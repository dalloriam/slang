use instructor::SysCall;

fn syscall_print(ro_block: &[u8], registers: &mut [i32]) -> bool {
    // Print something.
    // Expects the RO offset of the string in $0.

    let start_offset = registers[0] as usize;
    let mut end_offset = start_offset;
    while end_offset < ro_block.len() && ro_block[end_offset] != 0 {
        end_offset += 1;
    }

    // The VM expects the string to be UTF-8 encoded.
    let result = std::str::from_utf8(&ro_block[start_offset..end_offset]);
    match result {
        Ok(s) => {
            print!("{}", s);
        }
        Err(e) => {
            // TODO: Swap with logger.
            eprintln!("Error decoding string for print syscall: {:#?}", e);
            return false;
        }
    };

    return true;
}

pub fn execute_syscall(syscall: SysCall, ro_block: &[u8], registers: &mut [i32]) -> bool {
    match syscall {
        SysCall::NOP => true,
        SysCall::PRINT => syscall_print(ro_block, registers),
        SysCall::EXIT => false,
        _ => {
            eprintln!("Illegal Syscall. Terminating.",);
            false
        }
    }
}
