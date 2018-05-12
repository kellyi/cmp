global _start

section .data

teststring: db "helloworld", 0

section .text

strlen:
                               ; will take first and only arg from rdi
  xor rax, rax                 ; rax will hold the strlen.


.loop:
  cmp byte [rdi + rax], 0       ; check if current symbol is null-terminated
  je .end                       ; jump if we found the null terminator
  inc rax                       ; go to next symbol and increment the count
  jmp .loop                     ; restart the loop

.end:
  ret                           ; stop. rax will hold the return value

_start:
  mov rdi, teststring
  call strlen
  mov rdi, rax
  mov rax, 60
  syscall
