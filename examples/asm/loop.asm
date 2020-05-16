.data
.text
ld $0 #1
ld $1 #2
ld $3 #11

ld $11 #36

lp: add $0 $1 $0
eq  $0 $3
jeq @end
jmp @lp
end: hlt
