.data
.text
ld $v0 0x0003
ld $0 0x0004
syscall
move $v0 $1

ld $0 45
sw $0 0($1)

lw $10 0($1)

ld $v0 0x0004
move $1 $0
syscall

ld $v0 0x0002
syscall
