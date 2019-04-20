pub enum MetaCommandExecutionResult {
    SUCCESS,
    UNRECOGNIZED
}

pub fn is_meta_command(meta_command: &str) -> bool {
    return meta_command.starts_with(".");
}

pub fn execute_meta_command(meta_command: &str) -> MetaCommandExecutionResult {
    if meta_command == ".exit" {
        println!("bye");
        std::process::exit(0);
    }

    return MetaCommandExecutionResult::UNRECOGNIZED;
}