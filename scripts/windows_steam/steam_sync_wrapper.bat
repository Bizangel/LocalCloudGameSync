@echo off
setlocal
set "SYNC_EXE=C:\UtilityPrograms\bin\local_cloud_game_sync.exe"
set "SYNC_KEY=%~1"
shift

set "GAME_EXE=%~1"
shift
set "GAME_ARGS="
:collect
if "%~1"=="" goto run
set "GAME_ARGS=%GAME_ARGS% %~1"
shift
goto collect

:run
"%SYNC_EXE%" ui %SYNC_KEY%
set PRECODE=%ERRORLEVEL%
if not "%PRECODE%"=="0" exit /b %PRECODE%

if not defined GAME_EXE exit /b 1

start /wait "" "%GAME_EXE%" %GAME_ARGS%
set EXITCODE=%ERRORLEVEL%

"%SYNC_EXE%" ui %SYNC_KEY%
exit /b %EXITCODE%
