.data
.text
jmp @main
main:
sw $0 0[$ebp]
sw $0 4[$ebp]
sw $0 8[$ebp]
ld $8 0x0003
sw $8 4[$ebp]
popw $0
popw $0
popw $0
ld $v0 0x0002
syscall
