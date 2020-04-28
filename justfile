_bzl job target:
    bazel {{job}} {{target}}
    @echo "{{job}} complete"
    @notify-send "Bazel" "{{job}} complete for {{target}}"

build:
    @just _bzl build //...

_clippy target:
    @echo lint {{target}}
    @cd {{target}} && cargo check && cargo clippy

lint:
    @just _clippy repl
    @just _clippy slang/vm
    @just _clippy slang/instructor
    @just _clippy slang/assembler

test:
    @just _bzl test //...
