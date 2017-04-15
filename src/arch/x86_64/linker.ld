ENTRY(start)  /* start is the entry point the bootloader will jump to after loading the kernel */

SECTIONS {
    . = 1M;  /* sets the load address of the first section to 1 MiB */

    /* the executable will have two sections, .boot and .text */
    .boot :
    {
        /* ensure that the multiboot header is at the beginning */
        KEEP(*(.multiboot_header))
    }

    .text :
    {
        /* the .text output section contains all input sections named .text */
        *(.text)
    }
}