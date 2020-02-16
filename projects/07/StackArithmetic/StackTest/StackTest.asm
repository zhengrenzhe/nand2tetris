// push constant 17
@17
D=A
@SP
A=M
M=D
@SP
M=M+1


// push constant 17
@17
D=A
@SP
A=M
M=D
@SP
M=M+1


// eq
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=D-M
@label2dIsZero
D;JEQ
@label2dIsNotZero
D;JNE
(label2dIsZero)
D=-1
@label2end
0;JEQ
(label2dIsNotZero)
D=0
(label2end)
@SP
A=M
M=D
@SP
M=M+1


// push constant 17
@17
D=A
@SP
A=M
M=D
@SP
M=M+1


// push constant 16
@16
D=A
@SP
A=M
M=D
@SP
M=M+1


// eq
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=D-M
@label5dIsZero
D;JEQ
@label5dIsNotZero
D;JNE
(label5dIsZero)
D=-1
@label5end
0;JEQ
(label5dIsNotZero)
D=0
(label5end)
@SP
A=M
M=D
@SP
M=M+1


// push constant 16
@16
D=A
@SP
A=M
M=D
@SP
M=M+1


// push constant 17
@17
D=A
@SP
A=M
M=D
@SP
M=M+1


// eq
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=D-M
@label8dIsZero
D;JEQ
@label8dIsNotZero
D;JNE
(label8dIsZero)
D=-1
@label8end
0;JEQ
(label8dIsNotZero)
D=0
(label8end)
@SP
A=M
M=D
@SP
M=M+1


// push constant 892
@892
D=A
@SP
A=M
M=D
@SP
M=M+1


// push constant 891
@891
D=A
@SP
A=M
M=D
@SP
M=M+1




// push constant 891
@891
D=A
@SP
A=M
M=D
@SP
M=M+1


// push constant 892
@892
D=A
@SP
A=M
M=D
@SP
M=M+1




// push constant 891
@891
D=A
@SP
A=M
M=D
@SP
M=M+1


// push constant 891
@891
D=A
@SP
A=M
M=D
@SP
M=M+1




// push constant 32767
@32767
D=A
@SP
A=M
M=D
@SP
M=M+1


// push constant 32766
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1




// push constant 32766
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1


// push constant 32767
@32767
D=A
@SP
A=M
M=D
@SP
M=M+1




// push constant 32766
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1


// push constant 32766
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1




// push constant 57
@57
D=A
@SP
A=M
M=D
@SP
M=M+1


// push constant 31
@31
D=A
@SP
A=M
M=D
@SP
M=M+1


// push constant 53
@53
D=A
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


// push constant 112
@112
D=A
@SP
A=M
M=D
@SP
M=M+1








// push constant 82
@82
D=A
@SP
A=M
M=D
@SP
M=M+1





