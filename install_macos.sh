#!/bin/bash

# Install breaktimer_microphone_daemon
# to run at startup in the background

# Set project directory
PROJECT_DIR="."
BINARY_NAME="breaktimer_microphone_daemon"

# Step 1: Compile the Rust project
echo "Compiling Rust project..."
cd "$PROJECT_DIR" || exit 1
cargo build --release

# Step 2: Install the binary
echo "Installing binary..."
sudo cp "target/release/$BINARY_NAME" /usr/local/bin/
sudo chmod +x /usr/local/bin/$BINARY_NAME

# Step 3: Create Launch Agent plist
PLIST_DIR="$HOME/Library/LaunchAgents"
PLIST_FILE="$PLIST_DIR/com.user.$BINARY_NAME.plist"

mkdir -p "$PLIST_DIR"

cat > "$PLIST_FILE" <<EOL
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
  <dict>
    <key>Label</key>
    <string>com.user.$BINARY_NAME</string>
    <key>ProgramArguments</key>
    <array>
      <string>/usr/local/bin/$BINARY_NAME</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
  </dict>
</plist>
EOL

# Load the Launch Agent
echo "Loading Launch Agent..."
launchctl unload "$PLIST_FILE" 2>/dev/null
launchctl load "$PLIST_FILE"

echo "Setup complete. $BINARY_NAME will now run at startup."
