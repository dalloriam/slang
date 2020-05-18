.data
.text
ld $0 4
ld $v0 3
syscall

move $v0 $5
ld $v0 4
move $5 $0
syscall
