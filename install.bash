#!/bin/bash

bins=(flatten ttv-dir merge-data-folders scrub-non-images) 

cargo build --release

install_dir=/usr/local/bin

for bin in "${bins[@]}"
do
  sudo cp target/release/$bin $install_dir
done



