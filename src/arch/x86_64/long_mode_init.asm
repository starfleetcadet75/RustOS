global long_mode_start

section .text
bits 64
long_mode_start:
    ; zero out all data segment registers
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    ; call main
    extern rust_main
    call rust_main

    mov rax, 0x2f592f412f4b2f4f  ; print `OKAY` to screen
    mov qword [0xb8000], rax
    hlt