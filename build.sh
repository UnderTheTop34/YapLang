#!/bin/bash

cargo build > build_outputs.txt 2>&1

if [ $? -ne "0" ]; then
	cat build_outputs.txt
	printf "\033[31mBuild script (./build.sh): Build generated errors. Run 'cargo run' for more.\033[0m\n"
	rm build_outputs.txt
	exit
fi

rm build_outputs.txt

mv target/debug/YapLang ./yapl

