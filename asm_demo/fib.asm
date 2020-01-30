// calculate fibonacci number from 1 to 9 by user input


// wait press keyboard
(wait_keyboard_press)
@KBD
D=M // get keyboard value
@user_input
M=D // store keyboard value
@48
D=A
@user_input
M=M-D // convert user_input from keycode to number
@wait_keyboard_press
D;JEQ


// check whether input is between 1 and 10
@1
D=A
@current
M=D // set current = 1

@9
D=A
@max
M=D // set max = 9

(check_loop)
@current
D=M
@user_input
D=D-M
@calculate
D;JEQ // if current - user_input == 0, jump to calculate
@current
M=M+1 // current += 1
D=M
@max
D=M-D
@wait_keyboard_press
D;JLT // check whether current max than 9,
@check_loop
0;JEQ // start next loop


// calculate fibonacci
(calculate)
@0
D=A
@result
M=D

@1
D=A
@b
M=D

@user_input
M=M-1

@user_input
D=M
@n
M=D

(calculate_loop)
@n
D=M
@end_block
D;JLE
@result
D=M
@tmp
M=D
@b
D=M
@result
M=D
@b
D=M
@tmp
D=M+D
@b
M=D
@1
D=A
@n
M=M-D
@calculate_loop
0;JEQ


(end_block)
@end_block
0;JEQ
