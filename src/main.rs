use std::io;
use std::io::Write;

mod frontend;
use frontend::command_processor::meta_command_processor;
use frontend::command_processor::meta_command_processor::MetaCommandExecutionResult;
use frontend::command_processor::sql_command_processor::Executable;
use frontend::command_processor::sql_command_processor::Statement;
use frontend::command_processor::sql_command_processor::SQLCommandExecutionResult;
use frontend::command_processor::table::Table;

#[macro_use] extern crate scan_fmt;

fn read_input() -> Result<String, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    input = input.to_lowercase();
    Ok(input)
}

fn main() {
    let mut table = Table::new();

    loop {
        print!("SQLite> ");
        io::stdout().flush().expect("error flushing output");

        let mut command = read_input().unwrap();
        command = command.trim().to_string();

        if meta_command_processor::is_meta_command(&command) {
            match meta_command_processor::execute_meta_command(&command) {
                MetaCommandExecutionResult::SUCCESS => (),
                MetaCommandExecutionResult::UNRECOGNIZED => println!("unrecognized command '{}'", command)
            }

            continue;
        }

        let mut statement = Statement::new();
        match statement.prepare(&command) {
            SQLCommandExecutionResult::SUCCESS => (),
            SQLCommandExecutionResult::UNRECOGNIZED => {
                println!("unrecognized command '{}'", command);
                continue;
            }
        }

        statement.execute(&mut table);
    }
}