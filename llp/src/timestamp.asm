%include 'functions.asm'

SECTION .data
message db 'Seconds since Jan 01 1970: ', 0h

SECTION .text
global _start

_start:

  mov eax, message
  call sprint

  mov eax, 13                   ; SYS_TIME command
  int 80h

  call iprintlf
  call quit
