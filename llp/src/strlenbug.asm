global _start

section .data
teststring: db "abcdef", 0

section .text

strlen:
.loop:
  cmp byte [rdi+r13], 0
  je .end
  inc r13
  jmp .loop
.end:
  mov rax, r13
  ret

_start:
  mov rdi, teststring
  call strlen
  mov rdi, rax
  mov rax, 60
  syscall
