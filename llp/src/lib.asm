global _start

section .data
test_string: db "pizza", 10

section .text

exit:
  mov rax, 60                   ; set up the exit instruction
  syscall

string_length:
.loop:
  cmp byte [rdi+r13], 0
  je .end
  inc r13
  jmp .loop
.end:
  mov rax, r13
  ret

print_string:
  mov rsi, rdi                  ; set up string to print
  call string_length            ; get length of string to print
  mov rdx, rax                  ; store string length
  mov rax, 1                    ; set up the write syscall
  mov rdi, 1
  syscall
  ret

_start:
  mov rdi, test_string
  call print_string
  xor rdi, rdi
  call exit

; string_length:
;   xor rax, rax
;   ret

; print_string:
;   xor rax, rax
;   ret

; print_char:
;   xor rax, rax
;   ret

; print_newline:
;   xor rax, rax
;   ret

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
