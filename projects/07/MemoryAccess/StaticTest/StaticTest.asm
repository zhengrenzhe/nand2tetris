// push constant 111
@111
D=A
@SP
A=M
M=D
@SP
M=M+1


// push constant 333
@333
D=A
@SP
A=M
M=D
@SP
M=M+1


// push constant 888
@888
D=A
@SP
A=M
M=D
@SP
M=M+1


// pop static 8
@SP
M=M-1
@16
D=A
@8
D=D+A
@R13
M=D
@SP
AD=M
D=M
@R13
A=M
M=D


// pop static 3
@SP
M=M-1
@16
D=A
@3
D=D+A
@R13
M=D
@SP
AD=M
D=M
@R13
A=M
M=D


// pop static 1
@SP
M=M-1
@16
D=A
@1
D=D+A
@R13
M=D
@SP
AD=M
D=M
@R13
A=M
M=D


// push static 3
@16
D=A
@3
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1


// push static 1
@16
D=A
@1
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1


// sub
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=M-D
@SP
A=M
M=D
@SP
M=M+1


// push static 8
@16
D=A
@8
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1


// add
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=D+M
@SP
A=M
M=D
@SP
M=M+1

