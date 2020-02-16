// push constant 3030
@3030
D=A
@SP
A=M
M=D
@SP
M=M+1


@SP
M=M-1
A=M
D=M
@THIS
M=D


// push constant 3040
@3040
D=A
@SP
A=M
M=D
@SP
M=M+1


@SP
M=M-1
A=M
D=M
@THAT
M=D


// push constant 32
@32
D=A
@SP
A=M
M=D
@SP
M=M+1


// pop this 2
@SP
M=M-1
@THIS
D=M
@2
D=D+A
@R13
M=D
@SP
AD=M
D=M
@R13
A=M
M=D


// push constant 46
@46
D=A
@SP
A=M
M=D
@SP
M=M+1


// pop that 6
@SP
M=M-1
@THAT
D=M
@6
D=D+A
@R13
M=D
@SP
AD=M
D=M
@R13
A=M
M=D


@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1


@THAT
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


// push this 2
@THIS
D=M
@2
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


// push that 6
@THAT
D=M
@6
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

