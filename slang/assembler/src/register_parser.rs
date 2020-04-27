use nom::{
    bytes::complete::{tag, take_while},
    character::complete::{char, digit1},
    combinator::map_res,
    number::complete::double,
    sequence::preceded,
    IResult,
};

fn whitespace(i: &str) -> IResult<&str, &str> {
    let chars = " \t\r\n";
    take_while(move |c| chars.contains(c))(i)
}

fn integer_operand(i: &str) -> IResult<&str, i32> {
    map_res(
        preceded(whitespace, preceded(char('#'), digit1)),
        |int_val: &str| int_val.parse::<i32>(),
    )(i)
}

fn register(i: &str) -> IResult<&str, u8> {
    map_res(
        preceded(whitespace, preceded(char('$'), digit1)),
        |byte_val: &str| byte_val.parse::<u8>(),
    )(i)
}

#[test]
fn test_parse_register() {
    {
        let (_rest, reg) = register("$18").unwrap();
        assert_eq!(reg, 18);
    }
    {
        let (_rest, reg) = register(" $18").unwrap();
        assert_eq!(reg, 18);
    }

    {
        assert!(register("$400").is_err());
    }
}

#[test]
fn test_parse_operand() {
    {
        let (_rest, reg) = integer_operand("#10").unwrap();
        assert_eq!(reg, 10);
    }

    {
        let (_rest, reg) = integer_operand("#400").unwrap();
        assert_eq!(reg, 400);
    }

    {
        assert!(integer_operand("#asdf").is_err());
    }
}
