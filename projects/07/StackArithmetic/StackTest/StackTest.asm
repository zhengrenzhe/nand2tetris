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
@label2_true_block
D;JEQ
@label2_false_block
D;JNE
@label2_pass
0;JEQ
(label2_true_block)
D=-1
@label2_pass
0;JEQ
(label2_false_block)
D=0
(label2_pass)
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
@label5_true_block
D;JEQ
@label5_false_block
D;JNE
@label5_pass
0;JEQ
(label5_true_block)
D=-1
@label5_pass
0;JEQ
(label5_false_block)
D=0
(label5_pass)
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
@label8_true_block
D;JEQ
@label8_false_block
D;JNE
@label8_pass
0;JEQ
(label8_true_block)
D=-1
@label8_pass
0;JEQ
(label8_false_block)
D=0
(label8_pass)
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


// lt
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=M-D
@label11_true_block
D;JLT
@label11_false_block
D;JGE
@label11_pass
0;JEQ
(label11_true_block)
D=-1
@label11_pass
0;JEQ
(label11_false_block)
D=0
(label11_pass)
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


// lt
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=M-D
@label14_true_block
D;JLT
@label14_false_block
D;JGE
@label14_pass
0;JEQ
(label14_true_block)
D=-1
@label14_pass
0;JEQ
(label14_false_block)
D=0
(label14_pass)
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


// lt
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=M-D
@label17_true_block
D;JLT
@label17_false_block
D;JGE
@label17_pass
0;JEQ
(label17_true_block)
D=-1
@label17_pass
0;JEQ
(label17_false_block)
D=0
(label17_pass)
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





