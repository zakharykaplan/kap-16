use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{self, Display};
use std::str::FromStr;

use super::{Instruction, InstructionError, Op2};
use crate::{lex, uarch, util};

#[derive(Debug)]
pub struct Mul {
    op1: uarch,
    op2: Op2,
}

impl Display for Mul {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let label = "mul";
        let op1 = format!("r{}", self.op1);
        let op2 = match self.op2 {
            Op2::Reg(op2) => format!("r{}", op2),
            Op2::Imm(imm) => format!("{:#06x}", imm),
        };
        write!(f, "{} {}, {}", label, op1, op2)
    }
}

impl From<uarch> for Mul {
    fn from(word: uarch) -> Self {
        assert_eq!((word >> 12), 0b0111);
        Self {
            op1: (word & 0x0f00) >> 8,
            op2: match (word & 0x0080) == 0 {
                true => Op2::Reg(word & 0x000f),
                false => Op2::Imm(util::sign_extend::<7, { uarch::BITS }>(word & 0x007f)),
            },
        }
    }
}

impl From<Mul> for uarch {
    fn from(instr: Mul) -> Self {
        let mut word: uarch = 0;
        word |= 0b0111 << 12;
        word |= (instr.op1 << 8) & 0x0f00;
        word |= match instr.op2 {
            Op2::Reg(op2) => op2,
            Op2::Imm(imm) => 0x0080 | imm,
        } & 0x00ff;
        word
    }
}

impl FromStr for Mul {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Only operate on lowercase strings
        // (also creates an owned String from &str)
        let s = s.to_lowercase();
        // Split into constituent tokens
        let tokens = lex::tokenize(&s).ok_or(InstructionError::EmptyStr)?;
        // Ensure correct number of tokens
        match tokens.len().cmp(&4) {
            Ordering::Less => Err(InstructionError::MissingOps),
            Ordering::Equal => Ok(()),
            Ordering::Greater => Err(InstructionError::ExtraOps),
        }?;
        // Check instruction is correct
        (tokens[0] == "mul")
            .then(|| ())
            .ok_or(InstructionError::BadInstruction)?;
        // Parse op1
        let op1 = lex::parse_reg(&tokens[1])?;
        // Look for "," separator
        (tokens[2] == ",")
            .then(|| ())
            .ok_or(InstructionError::ExpectedSep)?;
        // Parse op2
        let op2 = tokens[3].parse()?;
        // Ensure validity of ops
        (op1 < 0x10)
            .then(|| ())
            .ok_or(InstructionError::InvalidOp)?;
        match op2 {
            Op2::Reg(reg) if reg < 0x10 => Ok(()),
            Op2::Imm(imm) if imm < 0x80 => Ok(()),
            _ => Err(InstructionError::InvalidOp),
        }?;
        // Create Self from parts
        Ok(Self { op1, op2 })
    }
}

impl Instruction for Mul {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sweep() {
        for mut word in 0x7000..=0x7fff {
            let instr = Mul::from(word);
            if let Op2::Reg(_) = instr.op2 {
                word &= 0xff8f;
            }
            let decoded: uarch = instr.into();
            assert_eq!(decoded, word);
        }
    }
}
