// square number
@3
D=A
@square_number
M=D

// square margin
@20
D=A
@margin
M=D

// square size
@30
D=A
@rect
M=D

// index
@0
D=A
@i
M=D

(draw_loop)




@i
D=M
D=D+1
M=D
@square_number
D=M-D
@draw_loop
D;JGE

(end)
@end
0;JEQ