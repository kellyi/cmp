global _start

section .text

exit:
  mov rax, 60                   ; set up the exit instruction
  syscall

_start:
  mov rdi, 10
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
