#!/bin/sh

rustc add.rs
gcc -Wall -o main main.c -L $(pwd) -ladd -lSystem -lpthread -lc -lm