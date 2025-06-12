#!/bin/bash

read -p "Insert the target directory:" target_dir

if [ ! -d "./$target_dir" ]; then
    echo "Error: target './$target_dir' doesn't exist"
    exit 1
fi

cd "./$target_dir"

mv ./rutgen_N ./rutgen
rutgen gen -p .
mv ./rutgen ./rutgen_N

mv ./rutgen_r ./rutgen
rutgen gen -p . -r
mv ./rutgen ./rutgen_r

mv ./rutgen_c ./rutgen
rutgen gen -p . -c
mv ./rutgen ./rutgen_c

mv ./rutgen_r_c ./rutgen
rutgen gen -p . -r -c 
rutgen fix -p .
mv ./rutgen ./rutgen_r_c

mv ./rutgen_r_c_o ./rutgen
rutgen gen -p . -r -c -o
rutgen fix -p .
mv ./rutgen ./rutgen_r_c_o