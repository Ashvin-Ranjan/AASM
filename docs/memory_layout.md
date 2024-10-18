## Memory Layout


### Instruction Block
- Starts: `0x00`
- Allocated upon startup
- Read-only

### Stack Block
- Starts: `0xFFFF`
- Allocated dynamically by stack pointer
- Read and write