%include 'functions.asm'

SECTION .text
global _start

_start:

  pop ecx

nextArg:
  cmp ecx, 0h
  jz noMoreArgs
  pop eax
  call sprintlf
  dec ecx
  jmp nextArg

noMoreArgs:
  call quit
