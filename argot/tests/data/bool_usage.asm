.data
.text
jmp @main
main:
sb $0 0[$ebp]
sb $0 1[$ebp]
ld $8 0x0001
sb $8 0[$ebp]
lb $8 0[$ebp]
sb $8 1[$ebp]
popb $0
popb $0
ld $v0 0x0002
syscall