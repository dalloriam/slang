.data
inta: .word 10
intb: .word 5

.text
lcw $0 @inta
lcw $1 @intb

add $0 $1 $2
