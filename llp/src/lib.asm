global _start

section .data
test_string: db "14", 0
test_char: db "c", 0
newline: db 0xa, 0

section .text

string_length:
  xor rax, rax                  ; set register to 0 to use as a counter
.loop:
  cmp byte [rdi+rax], 0         ; check whether anything remains to read
  je .end                       ; if nothing left, jump to end
  inc rax                       ; if another char is left, inc the counter
  jmp .loop                     ; restart loop
.end:
  ret

print_char:
  push rdi
  mov rdi, rsp
  call print_string
  pop rdi
  ret

print_newline:
  mov rdi, 10
  jmp print_char

print_string:
  push rdi
  call string_length
  pop rsi
  mov rdx, rax
  mov rax, 1
  mov rdi, 1
  syscall
  ret

print_uint:
  mov rax, rdi
  mov rdi, rsp
  push 0
  sub rsp, 16

  dec rdi
  mov r8, 10

.loop:
  xor rdx, rdx
  div r8
  or dl, 0x30
  dec rdi
  mov [rdi], dl
  test rax, rax
  jnz .loop

  call print_string

  add rsp, 24
  ret

print_int:
  test rdi, rdi
  jns print_uint
  push rdi
  mov rdi, '-'
  call print_char
  pop rdi
  neg rdi
  jmp print_uint

parse_int:
  mov al, byte [rdi]
  cmp al, '-'
  je .signed
  jmp parse_uint
.signed:
  inc rdi
  call parse_uint
  neg rax
  test rdx, rdx
  jz .error

  inc rdx
  ret
.error:
  xor rax, rax
  ret

parse_uint:
  mov r8, 10
  xor rax, rax
  xor rcx, rcx
.loop:
  movzx r9, byte [rdi+rcx]
  cmp r9b, '0'
  jb .end
  cmp r9b, '9'
  ja .end
  xor rdx, rdx
  mul r8
  and r9b, 0x0f
  add rax, r9
  inc rcx
  jmp .loop
.end:
  mov rdx, rcx
  ret

_start:
  mov rdi, test_string
  call parse_uint
  call print_uint
  call print_newline
  mov rdi, test_string
  call print_string
  call print_newline
  mov rdi, rax
  mov rax, 60
  syscall
