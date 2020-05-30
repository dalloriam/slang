.data
.text
jmp @main
main:
sw $0 0[$ebp]
ld $8 0x0003
neg $8
move $8 $8
sw $8 0[$ebp]
pop $0
ld $v0 0x0002
syscall
