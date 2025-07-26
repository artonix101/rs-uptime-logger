#!/bin/bash
set -euo pipefail

echo "NOTICE: This script will restore original system shutdown, reboot, and poweroff binaries in /sbin and /usr/bin."
echo "It will undo changes made by install_wrappers.sh."
echo

for cmd in shutdown reboot poweroff; do
    for target in /sbin; do #only /sbin and not /usr/bin because this happens automatically
        real_path="$target/$cmd.real"
        wrapped_path="$target/$cmd"

        if [ -f "$real_path" ]; then
            echo "Restoring original $wrapped_path from $real_path"
            sudo mv "$real_path" "$wrapped_path"
        else
            echo "No backup found for $wrapped_path — skipping"
        fi
    done
done

echo "Original binaries restored to /sbin and /usr/bin."
