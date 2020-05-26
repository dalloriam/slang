.data
.text
ld $0 1
ld $1 2
ld $3 1111

begin: add $0 $1 $0
eq  $0 $3
jeq @end
jmp @begin

end: ld $v0 2
syscall
