use instructor::Opcode;

use nom::{bytes::complete::tag, combinator::map, IResult};

pub fn load(input: &str) -> IResult<&str, Opcode> {
    map(tag("load"), |_f| Opcode::LOAD)(input)
}

#[cfg(test)]
mod tests {
    use super::{load, Opcode};

    #[test]
    fn test_opcode_load() {
        // Test valid opcode.
        let (_rest, opcode) = load("load").unwrap();
        assert_eq!(opcode, Opcode::LOAD);

        // Test invalid opcode.
        assert!(load("aold").is_err());
    }
}
