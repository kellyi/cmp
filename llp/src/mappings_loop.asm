SECTION .data
correct: dq -1

SECTION .text
global _start
_start:
  jmp _start
