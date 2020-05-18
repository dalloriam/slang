use instructor::SysCall;

use crate::constants::SYSCALL_REGISTER;
use crate::vm::VM;

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
            // TODO: Swap with logger.
            eprintln!("Error decoding string for print syscall: {:#?}", e);
            return false;
        }
    };

    return true;
}

fn syscall_alloc(vm: &mut VM) -> bool {
    let amt_to_allocate = vm.registers()[0] as u16;

    let heap = vm.heap_mut();
    let allocated_ptr = heap.alloc(amt_to_allocate as usize);

    vm.registers_mut()[SYSCALL_REGISTER] = allocated_ptr as i32; // OK b-c the heap is currently 16-bit.

    true
}

pub fn execute_syscall(syscall: SysCall, vm: &mut VM) -> bool {
    match syscall {
        SysCall::NOP => true,
        SysCall::CPRINT => syscall_cprint(vm),
        SysCall::EXIT => false,
        SysCall::ALLOC => syscall_alloc(vm),
        _ => {
            eprintln!("Illegal Syscall. Terminating.",);
            false
        }
    }
}
