#!/bin/sh
while true; do
  clear;
  cargo run --bin rust-test;
  git ls-files | xargs inotifywait -e close_write;
done
