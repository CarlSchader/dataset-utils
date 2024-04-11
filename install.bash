#!/bin/bash

bins=(flatten ttv-dir merge-data-folders scrub-non-images) 

cargo build --release

chown -R $SUDO_USER target

install_dir=/usr/local/bin

for bin in "${bins[@]}"
do
  cp target/release/$bin $install_dir
done



