// push constant 7
@7
D=A
@SP
M=D
A=A+1


// push constant 8
@8
D=A
@SP
M=D
A=A+1


// add
@SP
A=A-1
D=M
A=A-1
D=D+M
M=D
A=A+1

