.data
.text
jmp @main
main:
ld $8 0x0007
ld $9 0x0002
ld $10 0x0002
mul $9 $10 $9
ld $10 0x0002
add $9 $10 $9
mul $8 $9 $8
ld $v0 0x0002
syscall
