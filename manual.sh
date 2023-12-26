# 0_0_simple
echo "0_0_simple"
6991 pars -J 1 < ~cs6991/assign02_test_files/0_0_simple > outputref.txt
<tests/0_0_simple ./target/debug/pars -J 1 > outputme.txt
diff outputref.txt outputme.txt

# 0_1_simple_different
echo "0_1_simple_different"
6991 pars -J 1 < ~cs6991/assign02_test_files/0_1_simple_different > outputref.txt
<tests/0_1_simple_different ./target/debug/pars -J 1 > outputme.txt
diff outputref.txt outputme.txt 

# 0_2_many_commands
echo "0_2_many_commands"
6991 pars -J 1 < ~cs6991/assign02_test_files/0_2_many_commands > outputref.txt
<tests/0_2_many_commands ./target/debug/pars -J 1  > outputme.txt
diff outputref.txt outputme.txt

# 1_0_multiple_commands
echo "1_0_multiple_commands"
6991 pars -J 1 < ~cs6991/assign02_test_files/1_0_multiple_commands > outputref.txt
<tests/1_0_multiple_commands ./target/debug/pars -J 1 > outputme.txt
diff outputref.txt outputme.txt 

# 1_1_quoting
echo "1_1_quoting"
6991 pars -J 1 < ~cs6991/assign02_test_files/1_1_quoting > outputref.txt
<tests/1_1_quoting ./target/debug/pars -J 1 > outputme.txt
diff outputref.txt outputme.txt 

# 1_2_sleepy
echo "1_2_sleepy"
6991 pars -J 1 < ~cs6991/assign02_test_files/1_2_sleepy > outputref.txt
<tests/1_2_sleepy ./target/debug/pars -J 1 > outputme.txt
diff outputref.txt outputme.txt 

# 1_3_halting_problems
echo "1_3_halting_problems"
6991 pars -J 1 < ~cs6991/assign02_test_files/1_3_halting_problems > outputref.txt
<tests/1_3_halting_problems ./target/debug/pars -J 1 > outputme.txt
diff outputref.txt outputme.txt 

# 2_0_multiple_commands_2_threads
echo "2_0_multiple_commands_2_threads"
6991 pars -J 2 < ~cs6991/assign02_test_files/2_0_multiple_commands_2_threads > outputref.txt
<tests/2_0_multiple_commands_2_threads ./target/debug/pars -J 2 > outputme.txt
diff outputref.txt outputme.txt

# 2_1_multiple_commands_4_threads
echo "2_1_multiple_commands_4_threads"
6991 pars -J 4 < ~cs6991/assign02_test_files/2_1_multiple_commands_4_threads > outputref.txt
<tests/2_1_multiple_commands_4_threads ./target/debug/pars -J 4 > outputme.txt
diff outputref.txt outputme.txt 

# 2_2_multiple_commands_errors_4_threads
echo "2_2_multiple_commands_errors_4_threads"
6991 pars -J 4 < ~cs6991/assign02_test_files/2_2_multiple_commands_errors_4_threads > outputref.txt
<tests/2_2_multiple_commands_errors_4_threads ./target/debug/pars -J 4 > outputme.txt
diff outputref.txt outputme.txt 

# 2_3_ordering_good
echo "2_3_ordering_good"
6991 pars -J 2 < ~cs6991/assign02_test_files/2_3_ordering_good > outputref.txt
<tests/2_3_ordering_good ./target/debug/pars -J 2 > outputme.txt
diff outputref.txt outputme.txt 

# 3_1_halt_lazy_2_threads
echo "3_1_halt_lazy_2_threads"
6991 pars -J 2 --halt lazy < ~cs6991/assign02_test_files/3_1_halt_lazy_2_threads > outputref.txt
<tests/3_1_halt_lazy_2_threads ./target/debug/pars -J 2 --halt lazy > outputme.txt
diff outputref.txt outputme.txt 

# 3_2_halt_eager_2_threads
echo "3_2_halt_eager_2_threads"
6991 pars -J 2 --halt eager < ~cs6991/assign02_test_files/3_2_halt_eager_2_threads > outputref.txt
<tests/3_2_halt_eager_2_threads ./target/debug/pars -J 2 --halt eager > outputme.txt
diff outputref.txt outputme.txt 

# 3_3_halt_eager_3_threads
echo "3_3_halt_eager_3_threads"
6991 pars -J 3 --halt eager < ~cs6991/assign02_test_files/3_3_halt_eager_3_threads > outputref.txt
<tests/3_3_halt_eager_3_threads ./target/debug/pars -J 3 --halt eager > outputme.txt
diff outputref.txt outputme.txt 
