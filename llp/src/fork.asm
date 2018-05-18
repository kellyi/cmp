%include 'functions.asm'

SECTION .data
childMessage db 'This is the child process', 0h
parentMessage db 'This is the parent process', 0h

SECTION .text
global _start

_start:

  mov eax, 2                    ; invoke SYS_FORK
  int 80h                       ; call

  cmp eax, 0                    ; if eax is 0 we're in the child process
  jz child                      ; jump if in the child process

parent:
  mov eax, parentMessage        ; move parent message into register to print
  call sprintlf
  call quit

child:
  mov eax, childMessage         ; move child message into register to print
  call sprintlf
  call quit
