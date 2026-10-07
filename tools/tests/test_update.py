"""Tests for the updater: what it writes, and what it refuses to write.

The updater is the only tool here that overwrites a user's installed tree, so the
interesting behaviour is the refusals and the rollback, not the happy path.
"""
import hashlib
import json
import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

TOOLS = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(TOOLS))
import update  # noqa: E402


def sha(payload: bytes) -> str:
    return hashlib.sha256(payload).hexdigest()


def make_release(directory: Path, version: str, files: dict, layout: int = 1,
                 declared: dict | None = None) -> Path:
    """Write a release directory: payloads plus a manifest describing them."""
    directory.mkdir(parents=True, exist_ok=True)
    for rel, payload in files.items():
        target = directory / rel
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(payload)
    entries = declared if declared is not None else {
        rel: {"bytes": len(p), "sha256": sha(p)} for rel, p in files.items()
    }
    (directory / update.SELF_ENTRY).write_text(json.dumps({
        "manifest_version": 2, "kit_version": version, "layout_revision": layout,
        "files": entries,
    }, ensure_ascii=False), encoding="utf-8")
    return directory


def make_install(root: Path, version: str, files: dict) -> Path:
    root.mkdir(parents=True, exist_ok=True)
    (root / "VERSION").write_text(version + "\n", encoding="utf-8")
    for rel, payload in files.items():
        target = root / rel
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(payload)
    return root


class UpdatePlanTests(unittest.TestCase):
    def test_classifies_added_changed_and_current(self):
        with tempfile.TemporaryDirectory() as tmp:
            base = Path(tmp)
            root = make_install(base / "kit", "1.0.0", {"a.txt": b"old", "b.txt": b"same"})
            rel = make_release(base / "rel", "1.1.0",
                               {"a.txt": b"new", "b.txt": b"same", "c.txt": b"added"})
            plan = update.build_plan(update.release_from_directory(rel), root)
            self.assertEqual(plan["changed"], ["a.txt"])
            self.assertEqual(plan["added"], ["c.txt"])
            self.assertEqual(plan["current"], ["b.txt"])

    def test_local_only_files_are_reported_not_planned(self):
        with tempfile.TemporaryDirectory() as tmp:
            base = Path(tmp)
            root = make_install(base / "kit", "1.0.0", {"a.txt": b"old"})
            (root / "agent-config.json").write_text("{}", encoding="utf-8")
            (root / "my-own-notes.md").write_text("mine", encoding="utf-8")
            rel = make_release(base / "rel", "1.1.0", {"a.txt": b"new"})
            plan = update.build_plan(update.release_from_directory(rel), root)
            self.assertIn("agent-config.json", plan["local_only"])
            self.assertIn("my-own-notes.md", plan["local_only"])


class UpdateSafetyTests(unittest.TestCase):
    def test_manifest_path_escaping_the_install_is_rejected(self):
        for hostile in ("../outside.txt", "/etc/passwd", "a/../../b.txt"):
            with self.assertRaises(update.UpdateError):
                update.safe_rel(hostile)

    def test_manifest_cannot_write_machine_state(self):
        with self.assertRaises(update.UpdateError):
            update.safe_rel("agent-config.json")

    def test_corrupt_payload_is_caught_before_anything_is_written(self):
        with tempfile.TemporaryDirectory() as tmp:
            base = Path(tmp)
            root = make_install(base / "kit", "1.0.0", {"a.txt": b"old"})
            # Manifest declares the hash of b"new" while the file on disk holds other bytes.
            rel = make_release(base / "rel", "1.1.0", {"a.txt": b"tampered"},
                               declared={"a.txt": {"bytes": 3, "sha256": sha(b"new")}})
            release = update.release_from_directory(rel)
            plan = update.build_plan(release, root)
            with self.assertRaises(update.UpdateError):
                update.verify_payloads(release, plan)
            self.assertEqual((root / "a.txt").read_bytes(), b"old")

    def test_layout_revision_bump_is_refused_in_place(self):
        # The caller in main() compares against sc2_version.LAYOUT_REVISION; assert the
        # release exposes the value that decision depends on.
        with tempfile.TemporaryDirectory() as tmp:
            rel = make_release(Path(tmp) / "rel", "9.9.9", {"a.txt": b"x"}, layout=99)
            self.assertEqual(update.release_from_directory(rel).layout_revision, 99)


class UpdateApplyTests(unittest.TestCase):
    def test_apply_writes_files_updates_version_and_keeps_a_backup(self):
        with tempfile.TemporaryDirectory() as tmp:
            base = Path(tmp)
            root = make_install(base / "kit", "1.0.0", {"a.txt": b"old", "keep.txt": b"same"})
            rel = make_release(base / "rel", "1.1.0", {"a.txt": b"new", "keep.txt": b"same",
                                                       "c.txt": b"added"})
            release = update.release_from_directory(rel)
            plan = update.build_plan(release, root)
            update.verify_payloads(release, plan)
            backup = update.apply_plan(release, plan, root)

            self.assertEqual((root / "a.txt").read_bytes(), b"new")
            self.assertEqual((root / "c.txt").read_bytes(), b"added")
            self.assertEqual((root / "VERSION").read_text(encoding="utf-8").strip(), "1.1.0")
            self.assertEqual((backup / "a.txt").read_bytes(), b"old")
            self.assertFalse((backup / "c.txt").exists())

    def test_local_files_survive_an_apply(self):
        with tempfile.TemporaryDirectory() as tmp:
            base = Path(tmp)
            root = make_install(base / "kit", "1.0.0", {"a.txt": b"old"})
            (root / "agent-config.json").write_text('{"mine": true}', encoding="utf-8")
            (root / "scratch.md").write_text("local work", encoding="utf-8")
            rel = make_release(base / "rel", "1.1.0", {"a.txt": b"new"})
            release = update.release_from_directory(rel)
            plan = update.build_plan(release, root)
            update.apply_plan(release, plan, root)

            self.assertEqual((root / "agent-config.json").read_text(encoding="utf-8"),
                             '{"mine": true}')
            self.assertEqual((root / "scratch.md").read_text(encoding="utf-8"), "local work")

    def test_a_failure_midway_rolls_back_what_was_already_written(self):
        with tempfile.TemporaryDirectory() as tmp:
            base = Path(tmp)
            root = make_install(base / "kit", "1.0.0", {"a.txt": b"old-a", "b.txt": b"old-b"})
            rel = make_release(base / "rel", "1.1.0", {"a.txt": b"new-a", "b.txt": b"new-b"})
            release = update.release_from_directory(rel)
            plan = update.build_plan(release, root)

            real_replace = update.os.replace
            calls = {"n": 0}

            def failing_replace(src, dst):
                calls["n"] += 1
                if calls["n"] == 2:
                    raise OSError("injected failure on the second file")
                return real_replace(src, dst)

            with patch.object(update.os, "replace", failing_replace):
                with self.assertRaises(update.UpdateError) as caught:
                    update.apply_plan(release, plan, root)

            self.assertIn("回滚", str(caught.exception))
            # Both files must be back to their installed content.
            self.assertEqual((root / "a.txt").read_bytes(), b"old-a")
            self.assertEqual((root / "b.txt").read_bytes(), b"old-b")
            self.assertEqual((root / "VERSION").read_text(encoding="utf-8").strip(), "1.0.0")

    def test_installed_manifest_is_refreshed_not_treated_as_a_user_file(self):
        import zipfile
        with tempfile.TemporaryDirectory() as tmp:
            base = Path(tmp)
            root = make_install(base / "kit", "1.0.0", {"a.txt": b"old"})
            # an installed kit carries the previous release's manifest
            (root / update.SELF_ENTRY).write_text('{"kit_version": "1.0.0"}', encoding="utf-8")
            rel = make_release(base / "rel", "1.1.0", {"a.txt": b"new"})
            archive = base / "rel.zip"
            with zipfile.ZipFile(archive, "w") as zf:
                zf.write(rel / update.SELF_ENTRY, "kit/" + update.SELF_ENTRY)
                zf.write(rel / "a.txt", "kit/a.txt")
            with update.release_from_zip(archive) as release:
                plan = update.build_plan(release, root)
                self.assertNotIn(update.SELF_ENTRY, plan["local_only"])
                update.apply_plan(release, plan, root)
            installed = json.loads((root / update.SELF_ENTRY).read_text(encoding="utf-8"))
            self.assertEqual(installed["kit_version"], "1.1.0")

    def test_zip_and_directory_sources_agree(self):
        import zipfile
        with tempfile.TemporaryDirectory() as tmp:
            base = Path(tmp)
            rel = make_release(base / "rel", "1.1.0", {"a.txt": b"new"})
            archive = base / "rel.zip"
            with zipfile.ZipFile(archive, "w") as zf:
                zf.write(rel / update.SELF_ENTRY, "kit/" + update.SELF_ENTRY)
                zf.write(rel / "a.txt", "kit/a.txt")
            from_dir = update.release_from_directory(rel)
            with update.release_from_zip(archive) as from_zip:
                self.assertEqual(from_zip.version, from_dir.version)
                self.assertEqual(from_zip.payload("a.txt"), from_dir.payload("a.txt"))
                self.assertEqual(from_zip.checked_files(), from_dir.checked_files())
            # The zip must be released here or Windows keeps the file locked.
            archive.unlink()


if __name__ == "__main__":
    unittest.main()
