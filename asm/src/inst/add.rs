use std::fmt::{self, Display};

use super::Op2;
use crate::uarch;

#[derive(Debug)]
pub struct Add {
    op1: uarch,
    op2: Op2,
}

impl Display for Add {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let label = "add";
        let op1 = format!("r{}", self.op1);
        let op2 = match self.op2 {
            Op2::Op2(op2) => format!("r{}", op2),
            Op2::Imm(imm) => format!("{:#06x}", imm),
        };
        write!(f, "{} {}, {}", label, op1, op2)
    }
}

impl From<uarch> for Add {
    fn from(word: uarch) -> Self {
        assert_eq!((word >> 12), 0b1100);
        Self {
            op1: (word & 0x0f00) >> 8,
            op2: match (word & 0x0080) == 0 {
                true => Op2::Op2(word & 0x000f),
                false => Op2::Imm(word & 0x007f),
            },
        }
    }
}

impl From<Add> for uarch {
    fn from(instr: Add) -> Self {
        let mut word: uarch = 0;
        word |= 0b1100 << 12;
        word |= (instr.op1 << 8) & 0x0f00;
        word |= match instr.op2 {
            Op2::Op2(op2) => op2,
            Op2::Imm(imm) => 0x0080 | imm,
        } & 0x00ff;
        word
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sweep() {
        for mut word in 0xc000..=0xcfff {
            let instr = Add::from(word);
            if let Op2::Op2(_) = instr.op2 {
                word &= 0xff8f;
            }
            let decoded: uarch = instr.into();
            assert_eq!(decoded, word);
        }
    }
}
