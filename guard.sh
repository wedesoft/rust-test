#!/bin/sh
while true; do
  clear;
  cargo run --bin euler;
  git ls-files | xargs inotifywait -e close_write;
done
