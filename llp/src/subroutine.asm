; Any register your function needs shd have its current value put on the stack
; After the function has finished, restore registers with `pop`

SECTION .data
msg db "Hello world using a function!", 10

SECTION .text
global _start

_start:
  mov eax, msg
  call strlen
  mov edx, eax
  mov ecx, msg
  mov ebx, 1
  mov eax, 4
  int 80h

  mov ebx, 0
  mov eax, 1
  int 80h

strlen:
  push ebx
  mov ebx, eax

nextchar:
  cmp byte [eax], 0
  jz finished
  inc eax
  jmp nextchar

finished:
  sub eax, ebx
  pop ebx
  ret
