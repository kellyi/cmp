global _start

section .text
_start:
  mov rdi, 1                    ; set destination val to 1
  mov rax, 60                   ; set syscall instruction to 'exit' (with rdi)
  syscall                       ; make syscall -> rax(rdi)
