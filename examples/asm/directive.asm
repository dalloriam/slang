.data
hello: .asciiz "Hello everyone!"
world: .asciiz "Something else"

.text
ld $0 @hello
ld $v0 1
syscall

ld $0 @world
ld $v0 1
syscall

ld $v0 2
syscall
