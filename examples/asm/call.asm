.data
.text
main: ld $0 10
call @proc
add $0 $10 $5
ld $v0 2
syscall


proc: ld $10 32
ret
