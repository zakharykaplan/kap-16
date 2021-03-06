## Logical XOR

Uses:
`XOR`

Mnemonics:
- Logical e**X**clusive **OR**

Description:
> Perform a logical XOR operation.

Condition Codes:
- Cleared: carry, overflow
- Set: negative, zero

Examples:
```assembly
XOR Rx, Ry    ; set Rx <- Rx ^ Ry
XOR Rx, 0x2A  ; set Rx <- Rx ^ 0x2A
```

Format (Op2):
```
│15  12│11   8│ 7 │6   4│3    0│
┌──────┬──────┬───┬─────┬──────┐
│ 0001 │ XXXX │ 0 │ --- │ YYYY │
└──────┴──────┴───┴─────┴──────┘
```

Format (Imm):
```
│15  12│11   8│ 7 │6       0│
┌──────┬──────┬───┬─────────┐
│ 0001 │ XXXX │ 1 │ DDDDDDD │
└──────┴──────┴───┴─────────┘
```

Legend:
| Format   | Use              |
| -------- | ---------------- |
| `0`, `1` | Literal bit      |
| `D`      | Immediate data   |
| `X`      | Destination `Rx` |
| `Y`      | Source `Ry`      |
| `-`      | Unused           |
