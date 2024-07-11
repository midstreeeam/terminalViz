#!/bin/bash

# Check if dconf is available
if ! command -v dconf &> /dev/null
then
    echo "dconf command not found, skipping font size change"
    exit 0
fi

# Check arguments
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <font-size>"
    exit 1
fi

FONT_SIZE=$1
PROFILE=$(dconf read /org/gnome/terminal/legacy/profiles:/default | tr -d "'")

# Change font size
dconf write /org/gnome/terminal/legacy/profiles:/:$PROFILE/font "'Monospace $FONT_SIZE'"
