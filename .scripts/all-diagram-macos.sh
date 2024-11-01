#!/bin/bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PARENT_DIR="$(dirname "$SCRIPT_DIR")"
DOC_DIR="$PARENT_DIR/.doc"
DOT_NAME="all-diagram.dot"
IMG_NAME="all-diagram.svg"

mkdir -p "$DOC_DIR"

cargo modules dependencies --lib --no-externs --no-fns --no-sysroot --no-uses > "$DOC_DIR/$DOT_NAME"

if [ -f "$DOC_DIR/$DOT_NAME" ]; then
  dot -Tsvg "$DOC_DIR/$DOT_NAME" -o "$DOC_DIR/$IMG_NAME"
	   
   if [ -f "$DOC_DIR/$IMG_NAME" ]; then
     open -a Safari "$DOC_DIR/$IMG_NAME"
   else
     echo "Error: $IMG_NAME wurde nicht erstellt."
     exit 1
   fi
 else
   echo "Error: $DOT_NAME is missing."
   exit 1
 fi