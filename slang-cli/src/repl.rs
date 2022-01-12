use std::convert::TryInto;
use std::fs;
use std::io;
use std::io::Write;

use anyhow::Result;

use assembler::Assembler;
use vm::VM;

fn user_input(prompt: &str) -> Result<String> {
    let mut s = String::new();
    print!("{}", prompt);
    io::stdout().flush()?;
    io::stdin().read_line(&mut s)?;

    Ok(String::from(s.trim()))
}

fn run_command(vm: &mut VM, cmd: &str) -> Result<bool> {
    match cmd {
        ".load" => {
            vm.erase_program();
            let inpt = user_input("Enter path to file: ")?;
            println!("Loading bytecode from {}...", inpt);

            let data = fs::read(&inpt)?;
            let source_code = String::from_utf8(data)?;

            let mut asm = Assembler::new();
            let program = asm.assemble(&source_code)?;
            vm.load_bytecode(program)?;
            println!("Program loaded.")
        }
        ".program" => {
            println!("Instructions currently in VM memory:");
            for instruction in vm.program() {
                println!("{}", instruction);
            }
        }
        ".reg" => {
            println!("Current VM state:");
            let slice_ref: [i32; 32] = vm.registers()[0..32].try_into().unwrap();
            println!("{:#?}", slice_ref);
            // TODO: Print special registers also.
            println!("End of listing");
        }
        ".run" => {
            println!("Running to end of program...");
            vm.run();
        }
        ".unl" => {
            println!("Unloading program...");
            vm.erase_program();
        }
        ".exit" => {
            println!("Shutting down...");
            return Ok(false);
        }

        _ => {
            // Convert the hex to bytes, load it in the VM, and execute the instruction.
            let mut asm = Assembler::new();
            let program = asm.assemble(cmd)?;
            vm.load_bytecode(program)?;
            vm.run_once();
        }
    }

    Ok(true)
}

pub fn repl_loop() -> Result<()> {
    println!("SLang VM v0.1.0 REPL");
    let mut vm = VM::new();

    loop {
        let inpt = user_input(">> ")?;

        match run_command(&mut vm, &inpt) {
            Ok(v) => {
                if !v {
                    break;
                }
            }
            Err(e) => {
                eprintln!("REPL Error: {}", e);
            }
        }
    }
    Ok(())
}
