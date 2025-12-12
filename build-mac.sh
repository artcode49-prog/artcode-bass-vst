#!/bin/bash
set -e

echo "Building artcode SYNTH..."
cargo xtask bundle analog_synth --release

echo "Installing to ~/Library/Audio/Plug-Ins/VST3/"
mkdir -p ~/Library/Audio/Plug-Ins/VST3
cp -R "target/bundled/artcode SYNTH.vst3" ~/Library/Audio/Plug-Ins/VST3/

echo "Done! Restart your DAW to see artcode SYNTH."
