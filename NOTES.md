# Notes

### VGA Text Buffer

Address 0xb8000 contains the start of the VGA text buffer. It’s an array of screen characters that are displayed by the graphics card. A screen character consists of a 8 bit color code and a 8 bit ASCII character.    
 - 0x4f = white text on red background    
 - 0x52 = ASCII for 'R'   
 - 0x45 = ASCII for 'E'   
 - 0x3a = ASCII for ':'   
 - 0x20 = ASCII for ' '   

```mov dword [0xb8000], 0x4f524f4```   Puts 'ERR: ' into the VGA buffer
