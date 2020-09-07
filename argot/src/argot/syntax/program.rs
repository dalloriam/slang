use std::collections::HashMap;

use nom::{combinator::map, multi::many0, sequence::delimited, IResult};

use crate::syntax::{
    common::whitespace,
    function::{function_declaration, FunctionDeclaration},
};
use crate::visitor::{Visitable, Visitor};

#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub functions: HashMap<String, FunctionDeclaration>,
}

impl Visitable for Program {
    fn accept<V: Visitor>(&mut self, visitor: &mut V) -> V::Result {
        visitor.visit_program(self)
    }
}

pub fn program(i: &str) -> IResult<&str, Program> {
    map(
        many0(delimited(whitespace, function_declaration, whitespace)),
        |decls| {
            let mut hsh = HashMap::new();
            for decl in decls.into_iter() {
                hsh.insert(String::from(&decl.name), decl);
            }
            Program { functions: hsh }
        },
    )(i)
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::syntax::{
        program::program,
        types::{ArgumentList, Block, FunctionDeclaration, Program},
    };

    #[test]
    fn program_no_function() {
        let (rest, prg) = program("").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            prg,
            Program {
                functions: HashMap::new()
            }
        );
    }

    #[test]
    fn program_single_function() {
        let (rest, prg) = program("fn main() {}").unwrap();
        assert_eq!(rest, "");

        let mut fn_hash = HashMap::new();
        fn_hash.insert(
            String::from("main"),
            FunctionDeclaration {
                name: String::from("main"),
                return_type: None,
                block: Block::new(),
                args: ArgumentList::default(),
            },
        );
        assert_eq!(prg, Program { functions: fn_hash })
    }

    #[test]
    fn program_multi_function() {
        let (rest, prg) = program("fn main() {}\nfn hello() {}").unwrap();
        assert_eq!(rest, "");

        let mut fn_hash = HashMap::new();
        fn_hash.insert(
            String::from("main"),
            FunctionDeclaration {
                name: String::from("main"),
                return_type: None,
                block: Block::new(),
                args: ArgumentList::default(),
            },
        );
        fn_hash.insert(
            String::from("hello"),
            FunctionDeclaration {
                name: String::from("hello"),
                return_type: None,
                block: Block::new(),
                args: ArgumentList::default(),
            },
        );
        assert_eq!(prg, Program { functions: fn_hash })
    }
}
