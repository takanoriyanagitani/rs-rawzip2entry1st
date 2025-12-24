#!/bin/bash

izip=./sample.d/input.zip

genzip(){
	echo creating the sample input zip...
	mkdir -p ./sample.d

	echo hw1 > ./sample.d/hw1.txt
	echo hw2 > ./sample.d/hw2.txt

	find \
		./sample.d \
		-type f \
		-name '*.txt' |
		sort |
		zip \
			-0 \
			-@ \
			-T \
			-v \
			-o \
			./sample.d/input.zip

	echo
}

test -f "${izip}" || genzip

echo listing the contents of the zip...
unzip -lv "${izip}"
echo

echo showing the content of the 1st entry...
unzip -p "${izip}" sample.d/hw1.txt
echo

echo checking the content of the output with xxd...
cat "${izip}" |
	wazero \
		run \
		-timeout 10s \
		./rawzip2entry1st-opt.wasm -- --max-zip-bytes 65536 |
	xxd
