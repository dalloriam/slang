.data
.text
jmp @main
main:
ld $8 0x0002
ld $9 0x0003
mul $8 $9 $8
ld $v0 0x0002
syscall
