.data
.text
jmp @main
main:
sb $0 0[$ebp]
sb $0 1[$ebp]
ld $8 0x0001
ld $9 0x0001
and $8 $9
sb $8 0[$ebp]
ld $8 0x0000
ld $9 0x0001
or $8 $9
sb $8 1[$ebp]
lb $8 0[$ebp]
lb $9 1[$ebp]
and $8 $9
jez $8 @b
sb $0 0[$ebp]
ld $8 0x0001
sb $8 0[$ebp]
popb $0
b:
popb $0
popb $0
ld $v0 0x0002
syscall
