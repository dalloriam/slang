.data
inta: .word 0x0010
intb: .word 0x005

.text
lcw $0 @inta
lcw $1 @intb

add $0 $1 $2

ld $v0 2
syscall
