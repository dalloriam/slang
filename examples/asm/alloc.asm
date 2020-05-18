.data
.text
ld $v0 3
ld $0 4
syscall
move $v0 $1

ld $0 45
sw $0 0($1)

lw $10 0($1)

ld $v0 4
move $1 $0
syscall

ld $v0 2
syscall
