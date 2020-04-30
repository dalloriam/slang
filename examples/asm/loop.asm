ld $0 #1
ld $1 #2
ld $3 #11

ld $10 #16
ld $11 #36

add $0 $1 $0
eq  $0 $3
jeq $11
jmpb $10
hlt
