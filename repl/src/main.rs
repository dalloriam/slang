use std::io;
use std::io::Write;

use anyhow::Result;

use slang::VM;

fn user_input() -> Result<String> {
    let mut s = String::new();
    print!(">> ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut s)?;

    Ok(String::from(s.trim()))
}

fn parse_hex(i: &str) -> Result<Vec<u8>> {
    let split = i.split(' ').collect::<Vec<&str>>();
    let mut results = vec![];

    for hex_string in split {
        results.push(u8::from_str_radix(&hex_string, 16)?);
    }

    Ok(results)
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
            parse_hex(cmd)?.iter().for_each(|b| vm.add_byte(*b));
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
