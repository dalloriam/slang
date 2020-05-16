.data
hello: .asciiz "Hello everyone!"
.text
ld $v0 #1
syscall

ld $v0 #2
syscall
