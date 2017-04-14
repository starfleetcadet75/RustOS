; http://nongnu.askapache.com/grub/phcoder/multiboot.pdf
; Field         Type  Value
; magic number	u32   0xE85250D6
; architecture	u32   0 for i386, 4 for MIPS
; header length	u32   total header size, including tags
; checksum      u32   -(magic + architecture + header_length)
; tags          variable
; end tag       (u16, u16, u32) (0, 0, 8)
section .multiboot_header
header_start:
    dd 0xe85250d6  ; magic number (multiboot 2)
    dd 0  ; architecture 0 (protected mode i386)
    dd header_end - header_start  ; header length
    dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))  ; checksum

    ; optional multiboot tags

    ; required end tag
    dw 0  ; type
    dw 0  ; flags
    dd 8  ; size
header_end: