use instructor::Opcode;

use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult};

fn opcode_load(input: &str) -> IResult<&str, Opcode> {
    map(tag("load"), |_f| Opcode::LOAD)(input)
}

#[test]
fn test_opcode_load() {
    // Test valid opcode.
    let (_rest, opcode) = opcode_load("load").unwrap();
    assert_eq!(opcode, Opcode::LOAD);

    // Test invalid opcode.
    assert!(opcode_load("aold").is_err());
}
