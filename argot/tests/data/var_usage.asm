.data
.text
jmp @main
main:
sw $0 0[$ebp]
sw $0 4[$ebp]
ld $8 0x0003
sw $8 0[$ebp]
ld $8 0x0004
ld $9 0x0003
lw $10 0[$ebp]
add $9 $10 $9
mul $8 $9 $8
sw $8 4[$ebp]
lw $8 4[$ebp]
sw $8 0[$ebp]
popw $0
popw $0
ld $v0 0x0002
syscall
