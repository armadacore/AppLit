#!/bin/bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PARENT_DIR="$(dirname "$SCRIPT_DIR")"
DOC_DIR="$PARENT_DIR/.doc"
MD_NAME="project-structure.md"

mkdir -p "$DOC_DIR"

NO_COLOR=never cargo modules structure --lib --no-fns > "$DOC_DIR/$MD_NAME"

if [ -f "$DOC_DIR/$MD_NAME" ]; then
  echo "Success: $MD_NAME is created."
else
  echo "Error: $MD_NAME is missing."
  exit 1
fi