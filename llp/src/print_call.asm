section .data
demo1: dq 0x1122334455667788
demo2: db 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88
newline_char: db 10
codes: db '0123456789abcdef'

section .text
global _start

print_newline:
  mov rax, 1                    ; 'write' syscall identifier
  mov rdi, 1                    ; stdout file descriptor
  mov rsi, newline_char         ; set newline_char as the data source
  mov rdx, 1                    ; set the amount of bytes to write
  syscall
  ret

print_hex:
  mov rax, rdi
  mov rdi, 1
  mov rdx, 1
  mov rcx, 64                   ; amount to shift rax

iterate:
  push rax                      ; save initial rax value on stack
  sub rcx, 4                    ; ?
  sar rax, cl                   ; shift to 60, 56, 52, ... , 4, 0
  and rax, 0xf                  ; clear all the bits but the lowest 4
  lea rsi, [codes, rax]         ; hex digit char code
  mov rax, 1                    ; 'write' syscall identifier?
  push rcx                      ; syscall will break rcx
  syscall                       ; rax = 1 (31) -- the write identifier, rdi = 1 for stdout, rsi = char address
  pop rcx

  pop rax
  test rcx, rcx
  jnz iterate                   ; continue loop

  ret

_start:
  mov rdi, [demo1]
  call print_hex
  call print_newline

  mov rdi, [demo2]
  call print_hex
  call print_newline

  mov rax, 60
  xor rdi, rdi
  syscall
