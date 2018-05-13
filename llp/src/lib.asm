global _start

section .data
test_string: db "hello!", 0

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

_start:
  mov rdi, test_string
  call string_length
  mov rdi, rax
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
