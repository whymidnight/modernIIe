#! /usr/bin/env bash
./build.sh $1

echo "flashing..."
#elf2uf2-rs -d $1
probe-run --chip RP2040 $1
