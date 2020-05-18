use nom::{bytes::complete::take_while, IResult};

/// Eats {0-n} whitespace characters.
pub fn whitespace(i: &str) -> IResult<&str, &str> {
    let chars = " \t\r\n";
    take_while(move |c| chars.contains(c))(i)
}
