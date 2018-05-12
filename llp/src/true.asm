global _start

section .text
_start:
  mov rdi, 0                    ; set data to 0
  mov rax, 60                   ; set syscall to exit
  syscall                       ; make syscall -> rax(rdi)
