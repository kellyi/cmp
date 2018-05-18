%include 'functions.asm'

SECTION .data
fizz db 'Fizz', 0h
buzz db 'Buzz', 0h

SECTION .text
global _start

_start:

  mov esi, 0                    ; initialize checkfizz bool
  mov edi, 0                    ; initialize checkbuzz bool
  mov ecx, 0                    ; initialize counter

nextNumber:
  inc ecx                       ; increment our counter variable

.checkFizz:
  mov edx, 0                    ; clear edx register as a place to store remainder
  mov eax, ecx                  ; move value of counter into eax to divide
  mov ebx, 3                    ; mov fizz divisor into ebx
  div ebx                       ; divide what's in eax by fizz divisor, in ebx
  mov edi, edx                  ; move remainder into ebi
  cmp edi, 0                    ; check if the remainder is zero
  jne .checkBuzz                ; if it's not zero, jump to .checkBuzz op
  mov eax, fizz                 ; otherwise move the fizz string into eax
  call sprint                   ; print string in eax: fizz

.checkBuzz:
  mov edx, 0                    ; reset edx register to 0
  mov eax, ecx                  ; move value of counter into eax
  mov ebx, 5                    ; move buzz divisor into ebx
  div ebx                       ; divide eax by ebx, the buzz divisor
  mov esi, edx                  ; move remainder into esi
  cmp esi, 0                    ; check if remainder is zero
  jne .checkInt                 ; if remainder isn't zero, jump to just print int
  mov eax, buzz                 ; otherwise move the buzz string into eax
  call sprint                   ; print string in eax: buzz

.checkInt:
  cmp edi, 0                    ; check if we've printed fizz
  je .continue                  ; if so, jmp to .continue
  cmp esi, 0                    ; check if we've printed buzz
  je .continue                  ; if so, jmp to .continue
  mov eax, ecx                  ; still here? move the counter into eax
  call iprint                   ; print whats in eax as an integer

.continue:
  mov eax, 0ah                  ; add lf char to eax
  push eax                      ; push eax onto stack to print
  mov eax, esp                  ; get stack pointer to lf char
  call sprint                   ; print lf char
  pop eax                       ; pop lf char off stack
  cmp ecx, 100                  ; check whether we've reached 100
  jne nextNumber                ; if not, jump back to nextNumber
  call quit                     ; if so, quit
