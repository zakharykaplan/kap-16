use std::fmt::{self, Display};
use std::str::FromStr;

use super::{Instruction, Op2, ParseInstructionError};
use crate::{lex, uarch, util, WORDSIZE};

#[derive(Debug)]
enum Mode {
    Str = 0b0,
    Push = 0b1,
}

#[derive(Debug)]
pub struct Str {
    op1: uarch,
    op2: Op2,
    mode: Mode,
}

impl Display for Str {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let label = format!("{:?}", self.mode).to_lowercase();
        match self.mode {
            Mode::Str => {
                let op1 = format!("r{}", self.op1);
                let op2 = match self.op2 {
                    Op2::Op2(op2) => format!("r{}", op2),
                    Op2::Imm(imm) => format!("{:+#07x}", imm),
                };
                write!(f, "{} {}, &{}", label, op1, op2)
            }
            Mode::Push => {
                let op1 = format!("r{}", self.op1);
                write!(f, "{} {}", label, op1)
            }
        }
    }
}

impl From<uarch> for Str {
    fn from(word: uarch) -> Self {
        assert_eq!((word >> 12), 0b1101);
        Self {
            op1: (word & 0x0f00) >> 8,
            op2: match (word & 0x0080) == 0 {
                true => Op2::Op2(word & 0x000f),
                false => Op2::Imm(util::sign_extend::<8, { uarch::BITS }>(
                    (WORDSIZE as uarch) * (word & 0x007f),
                )),
            },
            mode: match ((word ^ 0x0040) & 0x00c0) == 0 {
                false => Mode::Str,
                true => Mode::Push,
            },
        }
    }
}

impl From<Str> for uarch {
    fn from(instr: Str) -> Self {
        let mut word: uarch = 0;
        word |= 0b1101 << 12;
        word |= (instr.op1 << 8) & 0x0f00;
        word |= match instr.op2 {
            Op2::Op2(op2) => match instr.mode {
                Mode::Str => op2,
                Mode::Push => 0x0040,
            },
            Op2::Imm(imm) => 0x0080 | (imm / (WORDSIZE as uarch)),
        } & 0x00ff;
        word
    }
}

impl FromStr for Str {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Only operate on lowercase strings
        // (also creates an owned String from &str)
        let s = s.to_lowercase();
        // Split into constituent tokens
        let tokens = lex::split(s).ok_or(Self::Err {})?;
        // Parse mode
        let mode = match &*tokens[0] {
            "str" => Mode::Str,
            "push" => Mode::Push,
            _ => Err(Self::Err {})?,
        };
        // Parse op1
        let op1 = match tokens[1].split_at(1) {
            ("r", reg) => Ok(reg.parse()?),
            _ => Err(Self::Err {}),
        }?;
        // Ensure validity of op1
        (op1 < 0x10).then(|| ()).ok_or(Self::Err {})?;
        // Parse for Mode::Str
        let op2 = match mode {
            Mode::Str => {
                // Look for "," separator
                ("," == tokens[2]).then(|| ()).ok_or(Self::Err {})?;
                // Parse op2
                let op2 = tokens[3].parse()?;
                // Ensure validity of op2
                match op2 {
                    Op2::Op2(reg) if reg < 0x10 => Ok(()),
                    Op2::Imm(imm) if imm < 0x80 => Ok(()),
                    _ => Err(Self::Err {}),
                }?;
                op2
            }
            Mode::Push => Op2::Op2(Default::default()),
        };
        // Create Self from parts
        Ok(Self { op1, op2, mode })
    }
}

impl Instruction for Str {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sweep() {
        for mut word in 0xd000..=0xdfff {
            let instr = Str::from(word);
            if let Op2::Op2(_) = instr.op2 {
                word &= 0xffcf;
            }
            if let Mode::Push = instr.mode {
                word &= 0xffc0;
            }
            let decoded: uarch = instr.into();
            assert_eq!(decoded, word);
        }
    }
}
