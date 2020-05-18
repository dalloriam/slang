use instructor::Opcode;

use nom::{character::complete::alpha1, combinator::map, sequence::delimited, IResult};

use crate::common::whitespace;

pub fn opcode(input: &str) -> IResult<&str, Opcode> {
    map(delimited(whitespace, alpha1, whitespace), |code| {
        Opcode::from(code)
    })(input)
}

#[cfg(test)]
mod tests {
    use super::{opcode, Opcode};

    #[test]
    fn test_opcode_load() {
        {
            // Test valid opcode.
            let (rest, op) = opcode("ld ").unwrap();
            assert_eq!(op, Opcode::LOAD);
            assert_eq!(rest, "");
        }

        {
            // Test invalid opcode.
            let (rest, op) = opcode("aold").unwrap();
            assert_eq!(op, Opcode::IGL);
            assert_eq!(rest, "");
        }
    }
}
