section .data
  msg db 'Hello world from an Assembly progam!', 10

section .text
global _start

_start:
  mov ebx, msg
  mov eax, ebx

nextchar:
  cmp byte [eax], 0
  jz finished
  inc eax
  jmp nextchar

finished:
  sub eax, ebx                  ; subtract address in ebx from the address in eax
  mov edx, eax                  ; eax is not the number of bytes in the string
  mov ecx, msg
  mov ebx, 1
  mov eax, 4
  int 80h

  mov ebx, 0
  mov eax, 1
  int 80h
