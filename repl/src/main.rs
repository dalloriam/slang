use std::io;
use std::io::Write;

use anyhow::Result;

use vm::VM;

fn user_input() -> Result<String> {
    let mut s = String::new();
    print!(">> ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut s)?;

    Ok(String::from(s.trim()))
}

fn run_command(vm: &mut VM, cmd: &str) -> Result<bool> {
    match cmd {
        ".program" => {
            println!("Instructions currently in VM memory:");
            for instruction in vm.program() {
                println!("{}", instruction);
            }
        }
        ".registers" => {
            println!("Current VM state:");
            println!("{:#?}", vm.registers());
            println!("End of listing");
        }
        ".exit" => {
            println!("Shutting down...");
            return Ok(false);
        }

        _ => {
            // Convert the hex to bytes, load it in the VM, and execute the instruction.
            let program = assembler::parse_program(cmd)?;
            program.get_bytecode().iter().for_each(|b| vm.add_byte(*b));
            vm.run_once();
        }
    }

    Ok(true)
}

fn repl_loop() -> Result<()> {
    println!("SLang VM v0.1.0 REPL");
    let mut vm = VM::new();

    loop {
        let inpt = user_input()?;

        match run_command(&mut vm, &inpt) {
            Ok(v) => {
                if !v {
                    break;
                }
            }
            Err(e) => {
                eprintln!("REPL Error: {}", e.to_string());
            }
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = repl_loop() {
        eprintln!("REPL Error: {}", e.to_string());
    }
}
