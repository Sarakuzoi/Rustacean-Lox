use std::{env, process::ExitCode};

use linefeed::{Interface, ReadResult};
use loxy_ferris::virtual_machine::InterpretResult;

// use loxy_ferris::virtual_machine::VM;

fn main() -> ExitCode {
    // let mut vm = VM::new();
    let args = env::args().collect::<Vec<String>>();
    println!("{:?}", args);

    // TODO: Lookup exit code for IO error
    if args.len() == 1 {
        return repl().unwrap_or_else(|_| ExitCode::from(1));
    } else if args.len() == 2 {
        return run_file(&args.get(0).unwrap()).unwrap_or_else(|_| ExitCode::from(1));
    } else {
        eprintln!("Usage: loxy-ferris [path]");
        return ExitCode::from(64);
    }
}

fn repl() -> Result<ExitCode, std::io::Error> {
    let reader = Interface::new("Loxy Ferris")?;
    reader.set_prompt("> ")?;
    while let ReadResult::Input(line) = reader.read_line()? {
        println!("Got input: {line}");
        interpret(line);
    }
    println!("Goodbye!");
    Ok(ExitCode::from(0))
}

fn run_file(path: &str) -> Result<ExitCode, std::io::Error> {
    let source = read_file(path)?;
    let result = interpret(source);

    if result == InterpretResult::INTERPRET_COMPILE_ERROR {
        return Ok(ExitCode::from(65));
    }
    if result == InterpretResult::INTERPRET_RUNTIME_ERROR {
        return Ok(ExitCode::from(70));
    }
    Ok(ExitCode::from(0))
}

fn read_file(_path: &str) -> Result<String, std::io::Error> {
    // TODO: Read file
    Ok(String::new())
}

fn interpret(code: String) -> InterpretResult {
    InterpretResult::INTERPRET_OK
}
