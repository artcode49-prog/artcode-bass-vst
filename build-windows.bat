@echo off
echo Building artcode SYNTH for Windows...

cargo xtask bundle analog_synth --release

if %ERRORLEVEL% NEQ 0 (
    echo Build failed!
    pause
    exit /b 1
)

echo.
echo Build successful!
echo.
echo To install, copy the VST3 folder to:
echo   C:\Program Files\Common Files\VST3\
echo.
echo Or run: copy /Y "target\bundled\artcode SYNTH.vst3" "C:\Program Files\Common Files\VST3\"
echo.
pause
