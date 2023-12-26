#!/bin/bash

echo "2_0_multiple_commands_2_threads"
6991 pars -J 2 < ~cs6991/assign02_test_files/2_0_multiple_commands_2_threads > outputref.txt
<tests/2_0_multiple_commands_2_threads ./target/debug/pars -J 2 > outputme.txt
diff outputref.txt outputme.txt