.data
.text
ld $0 10
ld $1 20

pushw $0
move $1 $0
popw $1

ld $v0 2
syscall
