#!/usr/bin/env bash

DEFAULT_DIR="${XDG_DATA_HOME:-$HOME/.local/share}/crabbybuddy/models"
mkdir -p $DEFAULT_DIR
uvx hf download google/gemma-4-E4B-it-qat-q4_0-gguf --local-dir $DEFAULT_DIR
