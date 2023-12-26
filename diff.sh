#!/bin/bash

mine="cargo run -- "
refer="6991 pars"
args="$@"

$mine $args < commands.txt > output1.txt
$refer $args < commands.txt > output1.txt

diff output1.txt output2.txt
