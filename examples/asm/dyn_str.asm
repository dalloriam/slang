.data
.text
loadstr: ld $v0 3
ld $0 6
syscall
move $v0 $1
move $1 $2

ld $10 4
ld $11 0
ld $20 97

strpushloop: sb $20 0($2)
inc $11
inc $20
inc $2
eq $10 $11
jeq @skip
jmp @strpushloop

skip: ld $5 10
sb $5 0($2)
inc $2
sb $30 0($2)
ld $v0 5
move $1 $0
syscall

teardown: ld $v0 4
move $0 $1
syscall
ld $v0 2
syscall
