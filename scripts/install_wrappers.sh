#!/bin/bash
set -e

WRAPPER="/usr/local/bin/wrappers" # Update this to where you compiled your Rust wrapper

for cmd in shutdown reboot poweroff; do
    # Move original binary if not already backed up
    if [ -f /sbin/$cmd ] && [ ! -f /sbin/$cmd.real ]; then
        sudo mv /sbin/$cmd /sbin/$cmd.real
    fi
    # Copy wrapper to /sbin/cmd
    sudo cp "$WRAPPER" /sbin/$cmd
    sudo chmod 755 /sbin/$cmd
done