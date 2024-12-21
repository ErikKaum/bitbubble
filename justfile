# Default recipe (this will be run when you type 'just')
default:
    @just --list

compile-mov-example:
    nasm ./examples/mov_example.asm -o ./examples/targets/mov_example 