use serde_json::Result;

use super::row::Row;

use crate::backend::os_interface;

pub struct Table {}

impl Table {
    pub fn new() -> Table {
        Table {}
    }

    pub fn add_row(&mut self, id: &String, row: &Row) -> Result<()> {
        os_interface::write(id, &serde_json::to_string(&row)?);
        Ok(())
    }

    pub fn select_row(&mut self, id: &String) {
        println!("{}" , os_interface::read(id));
    }

    /*pub fn print(&self) -> Result<()> {
        for i in 0..self.number_of_pages+1 {
            let page = &self.page_list[i];

            for j in 0..page.get_number_of_rows() {
                //TODO: handle returned result better, print function should not return a result
                println!("{}", serde_json::to_string(&page.get_row(j))?);
            }
        }

        Ok(())
    }*/
}