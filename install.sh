#!/bin/bash

bins=(flatten ttv-dir merge-data-folders scrub-non-images) 

cargo build --release

for bin in "${bins[@]}"
do
  cp target/release/$bin /usr/local/bin
done
