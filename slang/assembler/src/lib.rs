mod opcode_parser;
mod register_parser;

use instructor::Opcode;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op { code: Opcode },
}
