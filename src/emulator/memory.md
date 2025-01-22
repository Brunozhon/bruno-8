# Memory

The Bruno-8 has 65535 bytes of memory.

| Region            | Usage               |
|-------------------|---------------------|
| `0x0000`-`0x0FFF` | Reserved            |
| `0x1000`-`0x7FFF` | Program ROM         |
| `0x8000`-`0xEFFF` | General Purpose RAM |
| `0xF000`-`0xF0FF` | Stack               |
| `0xF100`-`0xF2FF` | Display (128x64)    |
| `0xF300`-`0xF301` | Peripheral Port 1   |
| `0xF302`-`0xF303` | Peripheral Port 2   |
| `0xF304`-`0xF305` | Peripheral Port 3   |
| `0xF306`-`0xF307` | Peripheral Port 4   |
| `0xF308`-`0xFFFB` | Unused              |
| `0xFFFC`          | Flags               |
| `0xFFFD`          | Stack Pointer       | 
| `0xFFFE`-`0xFFFF` | Program Counter     |
