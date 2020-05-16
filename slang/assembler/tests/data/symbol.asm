.data
.text
ld $0 #100
ld $1 #1
ld $2 #0
test: inc $0
neq $0 $2
jeq @test
hlt
