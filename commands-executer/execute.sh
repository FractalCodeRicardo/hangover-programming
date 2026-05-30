#!/usr/bin/env bash

set -euo pipefail

echo "Focus the target terminal..."
sleep 5

jq -c '.[]' commands.json | while read -r entry; do
    name=$(jq -r '.name' <<< "$entry")

    echo "=== $name ==="

    jq -r '.commands[]' <<< "$entry" | while read -r cmd; do
        ydotool type --key-delay 50 "$cmd"
        sleep 0.2
        ydotool key ENTER
        sleep 1
    done
done
