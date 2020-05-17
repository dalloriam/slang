.data
hello: .asciiz "Hello everyone!"

.text
ld $0 @hello
ld $v0 1
syscall

ld $v0 2
syscall
