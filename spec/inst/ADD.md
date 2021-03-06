## Arithmetic Add

Uses:
`ADD`

Mnemonics:
- **ADD**ition

Description:
> Perform an arithmetic addition.

Condition Codes:
| Flag     | Modified |
| -------- | -------- |
| Carry    | &check;  |
| Negative | &check;  |
| Overflow | &check;  |
| Zero     | &check;  |

Examples:
```assembly
ADD Rx, Ry    ; set Rx <- Rx + Ry
ADD Rx, 0d10  ; set Rx <- Rx + 10
```

Format (Op2):
```
│15  12│11   8│ 7 │6   4│3    0│
┌──────┬──────┬───┬─────┬──────┐
│ 1100 │ XXXX │ 0 │ --- │ YYYY │
└──────┴──────┴───┴─────┴──────┘
```

Format (Imm):
```
│15  12│11   8│ 7 │6       0│
┌──────┬──────┬───┬─────────┐
│ 1100 │ XXXX │ 1 │ DDDDDDD │
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
