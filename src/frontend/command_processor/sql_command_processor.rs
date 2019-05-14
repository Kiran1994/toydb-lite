use crate::data_structures::row::Row;
use crate::data_structures::table::Table;

pub enum SQLCommandExecutionResult {
    SUCCESS,
    UNRECOGNIZED    
}

enum StatementType {
    INSERT,
    SELECT,
    UNRECOGNIZED
}

pub struct Statement {
    statement_type: StatementType,
    row: Row
}

impl Statement {
    pub fn new() -> Statement {
        Statement{
            statement_type: StatementType::UNRECOGNIZED,
            row: Row::new()
        }
    }
}

pub trait Executable {
    fn prepare(&mut self, command: &str) -> SQLCommandExecutionResult;
    fn execute(&self, table: &mut Table);
}

fn derive_statement_type(command: &str) -> StatementType {
    if command.starts_with("insert") {
        return StatementType::INSERT;
    }
    else if command.starts_with("select") {
        return StatementType::SELECT;
    }

    return StatementType::UNRECOGNIZED;
}

fn insert_row(table: &mut Table, row: &Row) {
    table.add_row(&row.id, row);
}

fn select(table: &mut Table, id: &String) {
    table.select_row(id);
}

impl Executable for Statement {
    fn prepare(&mut self, command: &str) -> SQLCommandExecutionResult {
        match derive_statement_type(&command) {
            StatementType::INSERT => {
                self.statement_type = StatementType::INSERT;
                let (id, name, email) = scan_fmt!(command, "insert {} {} {}", String, String, String);
                //TODO: do error handling for invalid command syntax
                self.row.set_id(&id.unwrap());
                self.row.set_name(&name.unwrap());
                self.row.set_email(&email.unwrap());

                return SQLCommandExecutionResult::SUCCESS;
            },
            StatementType::SELECT => {
                self.statement_type = StatementType::SELECT;
                let id = scan_fmt!(command, "select {}", String);
                self.row.set_id(&id.unwrap());

                return SQLCommandExecutionResult::SUCCESS;
            },
            StatementType::UNRECOGNIZED => return SQLCommandExecutionResult::UNRECOGNIZED
        }
    }

    fn execute(&self, table: &mut Table) {
        match self.statement_type {
            StatementType::INSERT => insert_row(table, &self.row),
            StatementType::SELECT => select(table, &self.row.id),
            StatementType::UNRECOGNIZED => panic!("tried to execute unrecognized statement")
        }
    }
}