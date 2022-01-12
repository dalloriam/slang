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
        ensure!(data[0..4] == ELIS_HEADER_PREFIX, BadMagicNumberSnafu);
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
        ensure!(data.len() >= ELIS_HEADER_LENGTH, InvalidHeaderLengthSnafu);
        let header = Header::from_bytes(&data[0..ELIS_HEADER_LENGTH])?;

        ensure!(
            (header.ro_block_size + ELIS_HEADER_LENGTH) as usize <= data.len(),
            ReadOnlySectionTooLongSnafu
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

#[cfg(test)]
mod tests {
    use byteorder::{LittleEndian, WriteBytesExt};

    use super::{Program, ELIS_HEADER_LENGTH, ELIS_HEADER_PREFIX};

    #[test]
    pub fn minimum_valid_program() {
        let mut pad = vec![0; ELIS_HEADER_LENGTH - ELIS_HEADER_PREFIX.len()];
        let mut header = ELIS_HEADER_PREFIX.to_vec();
        header.append(&mut pad);

        let p = Program::new(header.clone()).unwrap();
        assert_eq!(p.header.ro_block_size, 0);
        assert_eq!(p.ro_block.len(), 0);
        assert_eq!(p.program_text.len(), 0);
    }

    #[test]
    pub fn small_program() {
        let mut pad = vec![0; ELIS_HEADER_LENGTH - ELIS_HEADER_PREFIX.len()];
        let mut header = ELIS_HEADER_PREFIX.to_vec();
        header.append(&mut pad);

        let mut instr = vec![1, 0, 0, 0];
        header.append(&mut instr);

        let p = Program::new(header).unwrap();
        assert_eq!(p.header.ro_block_size, 0);
        assert_eq!(p.ro_block.len(), 0);
        assert_eq!(p.program_text, vec![1, 0, 0, 0]);
    }

    #[test]
    pub fn bad_magic_number() {
        let mut pad = vec![0; ELIS_HEADER_LENGTH - ELIS_HEADER_PREFIX.len()];
        let mut header = vec![1, 2, 3, 4];
        header.append(&mut pad);

        assert!(Program::new(header).is_err());
    }

    #[test]
    pub fn header_too_short() {
        let mut pad = vec![0; 10];
        let mut header = ELIS_HEADER_PREFIX.to_vec();
        header.append(&mut pad);

        assert!(Program::new(header).is_err());
    }

    #[test]
    pub fn ro_section() {
        let mut pad = vec![0; ELIS_HEADER_LENGTH - (ELIS_HEADER_PREFIX.len() + 4)];
        let mut header = ELIS_HEADER_PREFIX.to_vec();
        header.resize(ELIS_HEADER_PREFIX.len() + 4, 0);
        (&mut header.as_mut_slice()[ELIS_HEADER_PREFIX.len()..])
            .write_u32::<LittleEndian>(4)
            .unwrap();
        header.append(&mut pad);

        let mut instr = vec![1, 2, 3, 4, 1, 0, 0, 0];
        header.append(&mut instr);

        let p = Program::new(header).unwrap();
        assert_eq!(p.header.ro_block_size, 4);
        assert_eq!(p.ro_block, vec![1, 2, 3, 4]);
        assert_eq!(p.program_text, vec![1, 0, 0, 0]);
    }
}
