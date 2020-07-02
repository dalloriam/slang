.data
.text
jmp @main
main:
sb $0 0[$ebp]
sb $0 1[$ebp]
ld $8 0x0000
sb $8 0[$ebp]
lb $8 0[$ebp]
ld $1 0x0001
ld $2 0x001f
move $8 $5
shr $8 $1
not $8
add $8 $5 $8
shr $8 $2
move $8 $8
sb $8 1[$ebp]
popb $0
popb $0
ld $v0 0x0002
syscall