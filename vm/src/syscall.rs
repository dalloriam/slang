use instructor::SysCall;

use crate::constants::SYSCALL_REGISTER;
use crate::VM;

fn syscall_cprint(vm: &VM) -> bool {
    // Print a constant.
    // Expects the RO offset of the string in $0.

    let start_offset = vm.registers()[0] as usize;
    let mut end_offset = start_offset;
    while end_offset < vm.ro_block().len() && vm.ro_block()[end_offset] != 0 {
        end_offset += 1;
    }

    // The VM expects the string to be UTF-8 encoded.
    let result = std::str::from_utf8(&vm.ro_block()[start_offset..end_offset]);
    match result {
        Ok(s) => {
            print!("{}", s);
        }
        Err(e) => {
            log::error!("Error decoding for print syscall: {:#?}", e);
            return false;
        }
    };

    true
}

fn syscall_prints(vm: &VM) -> bool {
    // Prints a string from memory.
    // Expects a ptr. to the beginning of the string in $0.

    let start_ptr = vm.registers()[0] as usize;
    let mut end_ptr = start_ptr;
    while end_ptr < vm.heap().memory().len() && vm.heap().memory()[end_ptr] != 0 {
        end_ptr += 1;
    }

    // The VM expects the string to be UTF-8 encoded.
    let result = std::str::from_utf8(&vm.heap().memory()[start_ptr..end_ptr]);
    match result {
        Ok(s) => {
            print!("{}", s);
        }
        Err(e) => {
            log::error!("Error decoding string for print syscall: {:#?}", e);
            return false;
        }
    };

    true
}

fn syscall_alloc(vm: &mut VM) -> bool {
    let amt_to_allocate = vm.registers()[0] as u16;

    let heap = vm.heap_mut();
    let allocated_ptr = heap.alloc(amt_to_allocate as usize);

    vm.registers_mut()[SYSCALL_REGISTER] = allocated_ptr as i32; // OK b-c the heap is currently 16-bit.

    true
}

fn syscall_free(vm: &mut VM) -> bool {
    let ptr_to_free = vm.registers()[0] as u16;

    let heap = vm.heap_mut();
    heap.free(ptr_to_free as usize);

    true
}

pub fn execute_syscall(syscall: SysCall, vm: &mut VM) -> bool {
    log::trace!("{:?}", syscall);
    match syscall {
        SysCall::NOP => true,
        SysCall::CPRINT => syscall_cprint(vm),
        SysCall::EXIT => false,
        SysCall::ALLOC => syscall_alloc(vm),
        SysCall::FREE => syscall_free(vm),
        SysCall::PRINTS => syscall_prints(vm),
        _ => {
            eprintln!("Illegal Syscall. Terminating.",);
            false
        }
    }
}
