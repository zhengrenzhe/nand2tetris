// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// Put your code here.

(loop)
@SCREEN
D=A

@pos
M=D

@24575
D=A
@max_pos
M=D

(draw_loop)
@KBD
D=M
@set_0
D;JNE
@set_1
D;JEQ

(set_0)
@0
D=A
@draw
0;JEQ

(set_1)
@1
D=A
@draw
0;JEQ

(draw)
@pos
A=M
M=D-1

@pos
M=M+1

@max_pos
D=M
@pos
D=D-M
@draw_loop
D;JGE

@loop
0;JEQ