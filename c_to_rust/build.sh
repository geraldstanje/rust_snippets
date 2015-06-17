#!/bin/sh

all() { 
  rustc add.rs
  gcc -Wall -o main main.c -L $(pwd) -ladd -lSystem -lpthread -lc -lm
}


clean() {
  rm libadd.a
  rm main
}

case $1 in all|clean) "$1" ;; *) printf >&2 '%s: unknown command\n' "$1"; exit 1;; esac