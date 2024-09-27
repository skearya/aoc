#!/bin/bash

nasm -f elf64 part_one.asm -o ./out/part_one.o && ld ./out/part_one.o -o ./out/part_one && ./out/part_one
