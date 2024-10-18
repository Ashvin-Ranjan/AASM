### Basic Info

Word Size: 16 bits \
Registers: 10, only first 8 are addressable (A, B, C, X, Y, Z, R, I, MAR, MDR)

Intruction Format:
S: `[Instr (6)]` \
R: `[Instr (6)][Size (1)][Register (3)][Register (3)][Register (3)]` \ 
I: `[Instr (6)][Size (1)][Register (3)][Register (3)][Immediate/Address (16)][Options (3)]` 

Instructions
#### `hlt`
Code: `0b000000` \
Format: `S` \
Description: Halts execution

#### `add`
Code: `0b000001` \
Format: `R` \
Description: `src3 = src1 + src2`
Code: `0b100001` \
Format: `I` \
Description: Sets `src2` to `src1*S + I`. `S` is equal to the 2 MSB of the options and `I` is either an immediate if the options
LSB is `0` or the value at `addr` if the options LSB is `1`

#### `sub`
Code: `0b000010` \
Format: `R` \
Description: `src3 = src1 - src2`
Code: `0b100010` \
Format: `I` \
Description: Sets `src2` to `src1*S - I`. `S` is equal to the 2 MSB of the options and `I` is either an immediate if the options
LSB is `0` or the value at `addr` if the options LSB is `1`

#### `shl`
Code: `0b000101` \
Format: `R` \
Description: `src3 = src1 << src2`
Code: `0b000101` \
Format: `R` \
Description: Sets `src2` to `src1*S<<I`. `S` is equal to the 2 MSB of the options and `I` is either an immediate if the options
LSB is `0` or the value at `addr` if the options LSB is `1`

`shr`
`slr`
`and`
`bor`
`xor`
`not`
`mul`
`div`
`jmp`
`jpe`
`jne`
`bre`
`bne`
`lod`
`mov`
`set`
`ste`
`sne`
`nop`: `0b111111`