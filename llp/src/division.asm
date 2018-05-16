%include 'functions.asm'

SECTION .data
message db ' remainder '

SECTION .text
global _start

_start:

  mov eax, 90
  mov ebx, 9
  div ebx
  call iprint
  mov eax, message
  call sprint
  mov eax, edx
  call iprintlf

  call quit
