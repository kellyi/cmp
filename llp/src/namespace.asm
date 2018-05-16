%include 'functions.asm'

SECTION .data
one db 'Jumping to finished label.', 0h
two db 'Inside a subroutine number: ', 0h
three db 'Inside a subroutine "finished".', 0h

SECTION .text
global _start

_start:

subroutineOne:
  mov eax, one
  call sprintlf
  jmp .finished

.finished:
  mov eax, two
  call sprint
  mov eax, 1
  call iprintlf

subroutineTwo:
  mov eax, one
  call sprintlf
  jmp .finished

.finished:
  mov eax, two
  call sprint
  mov eax, 2
  call iprintlf

  mov eax, one
  call sprintlf
  jmp finished

finished:
  mov eax, three
  call sprintlf
  call quit
