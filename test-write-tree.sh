#!/bin/sh
rm -rf test_dir
mkdir test_dir && cd test_dir 
cargo run -- init

# test_dir
# -> test_file_1.txt
# -> test_dir_1
# 	-> test_file_2.txt
# -> test_dir_2
# 	-> test_file_3.txt

echo "hello world" > test_file_1.txt
mkdir test_dir_1
echo "hello world" > test_dir_1/test_file_2.txt
mkdir test_dir_2
echo "hello world" > test_dir_2/test_file_3.txt

res=$(cargo run -- write-tree)

echo "$res"
# echo ""
# tree .git