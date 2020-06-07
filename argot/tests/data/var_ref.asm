.data
.text
jmp @main
main:
sw $0 0[$ebp]
sw $0 4[$ebp]
sw $0 8[$ebp]
ld $8 0x0007
sw $8 0[$ebp]
lw $8 0[$ebp]
ld $9 0x0006
mul $8 $9 $8
sw $8 4[$ebp]
lw $8 4[$ebp]
sw $8 8[$ebp]
popw $0
popw $0
popw $0
ld $v0 0x0002
syscall
