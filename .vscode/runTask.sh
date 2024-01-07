#!/usr/bin/env bash

input="$(pwd)/.vscode/input.txt"
output="$(pwd)/.vscode/output.txt"
if [[ $3 == "main.rs" ]]; then
    cd $4/..
    cargo run -- <$input >$output
    # dir="$(cut -d'/' -f-7 <<< "$1")";
    # name="$(cut -d'/' -f-2 <<< "$1")";
    # if [ -f "./target/debug/${name}" ]; then
    #     ./target/debug/$name < $input  > $output;
    #     rm ./target/debug/$name;
    # else
    #     echo "Check Log as fast as possible." > $output;
    # fi
    # rm ./target/debug/$name;
    # cut -d'/' -f-7 <<< $1;
    # echo $0 >> $output;
    # echo $1 >> $output;
    # echo $2 >> $output;
    # echo $3 >> $output;
    # echo $5 >> $output;
    # echo $dir > $output;
elif [[ $1==".cpp" ]]; then
    # g++ --version;
    /opt/homebrew/bin/g++-13 $1 -o $4/main.bin
    $4/main.bin <$input >$output
    rm $4/main.bin
else
    echo "Wrong file selected." >$output
    echo "Script exiting."
    exit -1
fi
