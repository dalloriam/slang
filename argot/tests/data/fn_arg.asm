.data
.text
jmp @main
main:
sw $0 0[$ebp]
sw $0 4[$ebp]
sw $0 8[$ebp]
ld $8 0x0006
sw $8 0[$ebp]
ld $8 0x0007
sw $8 4[$ebp]
lw $8 0[$ebp]
pushw $8
lw $8 4[$ebp]
pushw $8
call @multiply
popw $0
popw $0
ld $8 0x000a
sw $8 8[$ebp]
popw $0
popw $0
popw $0
ld $v0 0x0002
syscall
multiply:
sw $0 0[$ebp]
lw $8 -12[$ebp]
lw $9 -16[$ebp]
mul $8 $9 $8
sw $8 0[$ebp]
popw $0
ret