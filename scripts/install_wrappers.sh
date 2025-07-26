#!/bin/bash
set -euo pipefail

echo "WARNING: This script will replace system binaries (shutdown, reboot, poweroff) in /sbin and /usr/bin."
echo "This may affect system stability and shutdown/reboot functionality."
echo "Proceed AT YOUR OWN RISK!"
echo
read -p "Do you REALLY want to continue? [yes/N]: " answer

if [ "$answer" != "yes" ]; then
    echo "Aborted."
    exit 1
fi

WRAPPER="/usr/local/bin/wrappers"

for cmd in shutdown reboot poweroff; do
    for target in /sbin /usr/bin; do
        real_path="$target/$cmd.real"
        orig_path="$target/$cmd"

        echo "Backing up $orig_path to $real_path"
        #backup original binary if not already done
        if [ -f "$orig_path" ] && [ ! -f "$real_path" ]; then
            sudo mv "$orig_path" "$real_path"
        fi

        #copy wrapper to target path
        echo "Installing wrapper to $orig_path"
        sudo cp "$WRAPPER" "$orig_path"
        sudo chmod 755 "$orig_path"
    done
done

echo "Wrapper installed to /sbin and /usr/bin for shutdown, reboot, and poweroff."
