global start  ; exports label start (makes it public)

section .text  ; begin the .text section
bits 32  ; following lines need to be 32-bit instructions b/c CPU starts in protected mode
start:
    ; print `OK` to screen
    mov dword [0xb8000], 0x2f4b2f4f
    hlt