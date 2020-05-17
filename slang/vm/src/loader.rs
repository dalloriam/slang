use byteorder::{LittleEndian, ReadBytesExt};

use instructor::{ELIS_HEADER_LENGTH, ELIS_HEADER_PREFIX};

use snafu::{ensure, Snafu};

#[derive(Debug, Snafu)]
pub enum LoadError {
    BadMagicNumber,
    InvalidHeaderLength,
    ReadOnlySectionTooLong,
}

type Result<T> = std::result::Result<T, LoadError>;

// TODO: Use the header struct w/ serialization to *write* the header in the assembler as well.
pub struct Header {
    pub ro_block_size: usize,
}

impl Header {
    pub fn from_bytes(data: &[u8]) -> Result<Header> {
        ensure!(data[0..4] == ELIS_HEADER_PREFIX, BadMagicNumber);
        let ro_block_size = (&data[4..8]).read_u32::<LittleEndian>().unwrap() as usize;

        Ok(Header { ro_block_size })
    }
}

pub struct Program {
    pub header: Header,
    pub ro_block: Vec<u8>,
    pub program_text: Vec<u8>,
}

impl Program {
    pub fn new(data: Vec<u8>) -> Result<Program> {
        ensure!(data.len() >= ELIS_HEADER_LENGTH, InvalidHeaderLength);
        let header = Header::from_bytes(&data[0..ELIS_HEADER_LENGTH])?;

        ensure!(
            (header.ro_block_size + ELIS_HEADER_LENGTH) as usize <= data.len(),
            ReadOnlySectionTooLong
        );

        let ro_block = &data[ELIS_HEADER_LENGTH..ELIS_HEADER_LENGTH + header.ro_block_size];
        let program_text: Vec<u8> = data[ELIS_HEADER_LENGTH + header.ro_block_size..].into();

        Ok(Program {
            header,
            ro_block: ro_block.into(),
            program_text,
        })
    }
}
