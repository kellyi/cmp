global _start

section .data
test_string: db "pizza", 0
test_char: db "c", 0
newline: db 0xa, 0

section .text

exit:
  mov rax, 60                   ; set up the exit instruction
  syscall

string_length:
  xor rax, rax
.loop:
  cmp byte [rdi+rax], 0
  je .end
  inc rax
  jmp .loop
.end:
  ret

print_string:
  mov rsi, rdi                  ; set up string to print
  call string_length            ; get length of string to print
  mov rdx, rax                  ; store string length
  mov rax, 1                    ; set up the write syscall
  mov rdi, 1
  syscall
  ret

print_char:
  jmp print_string

print_newline:
  mov rdi, newline
  jmp print_char

_start:
  mov rdi, test_string
  call print_string
  call print_newline
  mov rdi, test_char
  call print_char
  call print_newline
  mov rdi, test_string
  call print_string
  call print_newline
  call print_newline
  call print_newline
  xor rdi, rdi
  call exit

; print_uint:
;   xor rax, rax
;   ret

; print_int:
;   xor rax, rax
;   ret

; read_char:
;   xor rax, rax
;   ret

; read_word:
;   xor rax, rax
;   ret

; parse_uint:
;   xor rax, rax
;   ret

; parse_int:
;   xor rax, rax
;   ret

; string_equals:
;   xor rax, rax
;   ret

; string_copy:
;   ret
