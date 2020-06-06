.data
.text
jmp @main
hello:
sb $0 0[$ebp]
ld $8 0x0001
sb $8 0[$ebp]
popb $0
ret
main:
sw $0 0[$ebp]
sw $0 4[$ebp]
ld $8 0x0003
sw $8 0[$ebp]
call @hello
ld $8 0x0004
sw $8 4[$ebp]
popw $0
popw $0
ld $v0 0x0002
syscall
