@echo off
rem Launch the installer GUI.
rem
rem ELECTRON_RUN_AS_NODE makes electron.exe behave as plain Node, in which case
rem require('electron') returns a path string instead of the API and the app dies
rem with "Cannot read properties of undefined (reading 'handle')".  Some toolchains
rem set it globally, so clear it for this process only.
set ELECTRON_RUN_AS_NODE=
cd /d "%~dp0"
if not exist "node_modules\electron\dist\electron.exe" (
  echo Electron is not installed yet.  Run:  npm install
  echo If the binary is missing afterwards, run:  cd node_modules\electron ^&^& node install.js
  exit /b 1
)
start "" "node_modules\electron\dist\electron.exe" .
