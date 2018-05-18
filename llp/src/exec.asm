%include 'functions.asm'

SECTION .data
command db '/bin/ls', 0h
arg1 db '-lah', 0h
arguments dd command
          dd arg1
          dd 0h
environment dd 0h

SECTION .text
global _start

_start:

  mov edx, environment          ; address of env var
  mov ecx, arguments            ; address of args to pass to command line
  mov ebx, command              ; address of file to exec
  mov eax, 11                   ; exec command
  int 80h

  call quit
