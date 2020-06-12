.data
.text
jmp @main
main:
sw $0 0[$ebp]
ld $8 0x0001
ld $9 0x0001
ld $10 0x0001
ld $11 0x0001
ld $12 0x0001
ld $13 0x0001
ld $14 0x0001
ld $15 0x0001
ld $16 0x0001
ld $17 0x0001
ld $18 0x0001
ld $19 0x0001
ld $20 0x0001
ld $21 0x0001
ld $22 0x0001
ld $23 0x0001
ld $24 0x0001
ld $25 0x0001
ld $26 0x0001
ld $27 0x0001
ld $28 0x0001
ld $29 0x0001
ld $30 0x0001
ld $31 0x0001
ld $7 0x0001
pushw $7
ld $7 0x0001
pushw $7
ld $7 0x0001
pushw $7
popw $0
popw $1
add $1 $0 $0
pushw $0
popw $0
popw $1
add $1 $0 $0
pushw $0
popw $0
add $31 $0 $31
add $30 $31 $30
add $29 $30 $29
add $28 $29 $28
add $27 $28 $27
add $26 $27 $26
add $25 $26 $25
add $24 $25 $24
add $23 $24 $23
add $22 $23 $22
add $21 $22 $21
add $20 $21 $20
add $19 $20 $19
add $18 $19 $18
add $17 $18 $17
add $16 $17 $16
add $15 $16 $15
add $14 $15 $14
add $13 $14 $13
add $12 $13 $12
add $11 $12 $11
add $10 $11 $10
add $9 $10 $9
add $8 $9 $8
sw $8 0[$ebp]
popw $0
ld $v0 0x0002
syscall