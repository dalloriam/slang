.data
hello: .asciiz "Hello everyone!"
world: .asciiz "Something else"

.text
ld $0 @hello
ld $v0 0x0001
syscall

ld $0 @world
ld $v0 0x0001
syscall

ld $v0 0x0002
syscall
