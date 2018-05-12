global _start

section .data
message: db 'hello, world!', 10

section .text
_start:
  mov rax, 1                     ; system call number should be stored in rax
  mov rdi, 1                     ; argument one in rdi: where to write, or, file descriptor
  mov rsi, message               ; argument two in rsi: where does the string start?
  mov rdx, 14                    ; argument three in rdx: how many bytes to write
  syscall                        ; invoke a system call, which gathers the previously set args
