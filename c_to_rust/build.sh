#!/bin/sh

all() { 
  rustc src/add.rs
  rustc src/hello.rs
  gcc -Wall -o main src/main.c -L $(pwd) -ladd -lhello -lSystem -lpthread -lc -lm
}


clean() {
  rm libadd.a
  rm libhello.a
  rm main
}

case $1 in all|clean) "$1" ;; *) printf >&2 '%s: unknown command\n' "$1"; exit 1;; esac