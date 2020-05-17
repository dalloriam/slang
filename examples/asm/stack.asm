.data
.text
ld $0 10
ld $1 20

push $0
move $1 $0
pop $1

ld $v0 2
syscall
