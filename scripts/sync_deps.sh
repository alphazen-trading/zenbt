#!/bin/bash

# Extract dependencies from pyproject.toml
deps=$(awk '/dependencies = \[/,/\]/ {
    if ($0 ~ /^[[:space:]]*"[^"]+",?$/) {
        gsub(/^[[:space:]]*"|",[[:space:]]*$/, "")
        gsub(/[<>=!~].*/, "")
        print $0
    }
}' pyproject.toml | tr '\n' ' ' | sed -e 's/[[:space:]]*$//')

if [ -n "$deps" ]; then
    echo "Found dependencies: $deps"
    echo "Removing dependencies..."
    eval "uv remove $deps"
    echo "Installing dependencies..."
    eval "uv add $deps"
else
    echo "No dependencies found in pyproject.toml"
    exit 1
fi
