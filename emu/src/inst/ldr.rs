use std::fmt::{self, Display};

use super::{Instruction, Op2};
use crate::{iarch, uarch, util, Processor, WORDSIZE};

#[derive(Debug)]
pub struct Ldr {
    op1: uarch,
    op2: Op2,
    pop: bool,
}

impl Display for Ldr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.pop {
            let label = "pop";
            let op1 = format!("r{}", self.op1);
            write!(f, "{} {}", label, op1)
        } else {
            let label = "ldr";
            let op1 = format!("r{}", self.op1);
            let op2 = match self.op2 {
                Op2::Reg(op2) => format!("r{}", op2),
                Op2::Imm(imm) => format!("{:+#07x}", imm),
            };
            write!(f, "{} {}, *{}", label, op1, op2)
        }
    }
}

impl From<uarch> for Ldr {
    fn from(word: uarch) -> Self {
        assert_eq!((word >> 12), 0b1011);
        Self {
            op1: (word & 0x0f00) >> 8,
            op2: match (word & 0x0080) == 0 {
                true => Op2::Reg(word & 0x000f),
                false => Op2::Imm(util::sign_extend::<8, { uarch::BITS }>(
                    (WORDSIZE as uarch) * (word & 0x007f),
                )),
            },
            pop: ((word ^ 0x0040) & 0x00c0) == 0,
        }
    }
}

impl From<Ldr> for uarch {
    fn from(instr: Ldr) -> Self {
        let mut word: uarch = 0;
        word |= 0b1011 << 12;
        word |= (instr.op1 << 8) & 0x0f00;
        word |= match instr.op2 {
            Op2::Reg(op2) => match instr.pop {
                false => op2,
                true => 0x0040,
            },
            Op2::Imm(imm) => 0x0080 | (imm / (WORDSIZE as uarch)),
        } & 0x00ff;
        word
    }
}

impl Instruction for Ldr {
    fn execute(&self, proc: &mut Processor) {
        // Compute result
        let res = match self.op2 {
            Op2::Reg(op2) => match self.pop {
                false => *proc.regs[op2],
                true => *proc.regs[13],
            },
            Op2::Imm(imm) => (*proc.regs[15] as iarch + imm as iarch) as uarch,
        };
        // Increment frame pointer
        if self.pop {
            *proc.regs[13] += WORDSIZE as uarch;
        }
        // Set result
        *proc.regs[self.op1] = proc.ram[res];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sweep() {
        for mut word in 0xb000..=0xbfff {
            let instr = Ldr::from(word);
            if let Op2::Reg(_) = instr.op2 {
                word &= 0xffcf;
            }
            if instr.pop {
                word &= 0xffc0;
            }
            let decoded: uarch = instr.into();
            assert_eq!(decoded, word);
        }
    }
}
