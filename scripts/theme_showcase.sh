#!/bin/bash

# GlowDoc Theme Showcase Script
# Cycles through themes, builds site, captures screenshots with light/dark modes

set -e

THEMES=("vibrant" "default" "purple")
CONFIG_FILE="docs/config.yaml"
SCREENSHOTS_DIR="screenshots"

# Create screenshots directory
mkdir -p "$SCREENSHOTS_DIR"

echo "Checking for 'uv' installation..."

# Attempt to run 'uv --version' and capture its exit status.
# Redirecting stdout and stderr to /dev/null to keep the output clean.
uv --version > /dev/null 2>&1

# Check the exit status of the previous command.
if [ $? -eq 0 ]; then
  echo "'uv' is installed and callable."
  # Optionally, you can display the version if it was successful.
  uv --version
else
  echo "Error: 'uv' is not installed or not callable."
  echo "Please ensure 'uv' is installed and available in your system's PATH."
  exit 1 # Exit with a non-zero status to indicate failure
fi

echo "Checking for 'uv' installation..."

# Attempt to run 'uv --version' and capture its exit status.
# Redirecting stdout and stderr to /dev/null to keep the output clean.
htop --version > /dev/null 2>&1

# Check the exit status of the previous command.
if [ $? -eq 0 ]; then
  echo "'htop' is installed and callable."
  # Optionally, you can display the version if it was successful.
  uv --version
else
  echo "Error: 'htop' is not installed or not callable."
  echo "Please ensure 'htop' is installed and available in your system's PATH."
  exit 1 # Exit with a non-zero status to indicate failure
fi

echo "🎨 Starting GlowDoc theme showcase..."

for theme in "${THEMES[@]}"; do
    echo "📋 Setting theme to: $theme"
    
    # Update theme in config.yaml
    sed -i.bak "s/theme: .*/theme: $theme/" "$CONFIG_FILE"
    
    echo "🔨 Building with $theme theme..."
    cargo run --release
    
    echo "🌐 Taking Dark mode screenshot"
    osascript -e 'tell app "System Events" to tell appearance preferences to set dark mode to true'
    htop --png -w "1200,800" single index.html $SCREENSHOTS_DIR/dark_$theme.png

    # wait 1 second while everything updates
    # sleep 1

    echo "☀️ Taking Light mode screenshot"
    osascript -e 'tell app "System Events" to tell appearance preferences to set dark mode to false'
    htop --png -w "1200,800" single index.html $SCREENSHOTS_DIR/light_$theme.png

    echo "✅ Completed $theme theme grab"
    echo "---"
done

# Revert back to Dark Mode
osascript -e 'tell app "System Events" to tell appearance preferences to set dark mode to true'

# Restore original theme
echo "🔄 Restoring original theme..."
mv "$CONFIG_FILE.bak" "$CONFIG_FILE"

echo "🎉 Theme showcase complete!"
echo "📁 Screenshots saved to: $SCREENSHOTS_DIR/"
echo "🖼️  Generated files:"
for theme in "${THEMES[@]}"; do
    echo "   - ${theme}_light.png"
    echo "   - ${theme}_dark.png"
done

# Create the overlay image
uv run ./scripts/create_overlay.py

# remove the screenshots directory
rm -rf $SCREENSHOTS_DIR

# remove the backup
rm -rf ./docs/config.yaml.backup

# move the diagonal_comparison into the docs folder
mv ./diagonal_comparison.png ./docs/diagonal_comparison.png

