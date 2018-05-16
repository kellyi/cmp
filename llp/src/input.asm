%include 'functions.asm'

SECTION .data
one db 'Please enter your name: ', 0h
two db 'Hello, ', 0h

SECTION .bss
sinput: resb 255                ; reserve 255 bytes for user input

SECTION .text
global _start

_start:

  mov eax, one
  call sprint

  mov edx, 255
  mov ecx, sinput
  mov ebx, 0
  mov eax, 3                    ; read system call
  int 80h

  mov eax, two
  call sprint

  mov eax, sinput
  call sprint

  call quit
