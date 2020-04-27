_bzl job target:
    bazel {{job}} {{target}}
    @echo "{{job}} complete"
    @notify-send "Bazel" "{{job}} complete for {{target}}"

build:
    @just _bzl build //...

test:
    @just _bzl test //...
