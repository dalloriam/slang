use nom::{
    character::complete::char,
    combinator::map,
    multi::separated_list,
    sequence::{delimited, tuple},
    IResult,
};

use crate::syntax::{common::whitespace, var_decl::identifier};

#[derive(Clone, Debug, PartialEq)]
pub struct Argument {
    pub arg_type: String,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ArgumentList {
    pub arguments: Vec<Argument>,
}

impl Default for ArgumentList {
    fn default() -> Self {
        ArgumentList {
            arguments: Vec::new(),
        }
    }
}

pub fn argument_list(i: &str) -> IResult<&str, ArgumentList> {
    map(
        delimited(
            char('('),
            separated_list(
                char(','),
                delimited(whitespace, tuple((identifier, identifier)), whitespace),
            ),
            char(')'),
        ),
        |args| ArgumentList {
            arguments: args
                .into_iter()
                .map(|(arg_type, name)| Argument {
                    arg_type: String::from(arg_type),
                    name: String::from(name),
                })
                .collect(),
        },
    )(i)
}

#[cfg(test)]
mod tests {
    use super::{argument_list, Argument, ArgumentList};

    #[test]
    fn empty_list() {
        let (rest, arglist) = argument_list("()").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            arglist,
            ArgumentList {
                arguments: Vec::new()
            }
        );
    }

    #[test]
    fn single_arg() {
        let (rest, arglist) = argument_list("(bool b)").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            arglist,
            ArgumentList {
                arguments: vec![Argument {
                    name: String::from("b"),
                    arg_type: String::from("bool")
                }]
            }
        )
    }
}
