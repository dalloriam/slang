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
lw $0 0[$ebp]
move $0 $10
add $9 $10 $0
move $0 $9
mul $8 $9 $0
move $0 $8
sw $8 4[$ebp]
pop $0
pop $0
ld $v0 0x0002
syscall
