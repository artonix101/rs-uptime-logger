#!/bin/bash
set -e

for cmd in shutdown reboot poweroff; do
    if [ -f /sbin/$cmd.real ]; then
        sudo mv /sbin/$cmd.real /sbin/$cmd
    fi
done