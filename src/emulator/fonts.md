# Fonts

Fonts are stored in four addresses. The high four bits are the odd scan lines and the low four bits are the even scan lines.

## Example
### In Memory
```
0x0004 0b01101001
0x0005 0b10011111
0x0006 0b10011001
0x0007 0b10011001
```
### Result
```
  ████
██    ██
██    ██
██    ██
████████
██    ██
██    ██
██    ██
██    ██
```
> **Note**
> 
> `██` = 1 pixel on
> 
> two spaces = 1 pixel off