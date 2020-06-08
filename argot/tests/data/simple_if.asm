.data
.text
jmp @main
main:
ld $8 0x0001
jez $8 @condition
sw $0 0[$ebp]
ld $8 0x0003
sw $8 0[$ebp]
popw $0
condition:
ld $v0 0x0002
syscall