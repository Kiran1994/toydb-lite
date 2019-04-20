pub struct Row {
    id: String,
    name: String,
    email: String
}

impl Row {
    pub fn new() -> Row {
        Row {
            id: "".to_string(),
            name: "".to_string(),
            email: "".to_string()
        }
    }

    pub fn set_id(&mut self, id: &String) {
        self.id = id.to_string();
    }

    pub fn set_name(&mut self, name: &String) {
        self.name = name.to_string();
    }

    pub fn set_email(&mut self, email: &String) {
        self.email = email.to_string();
    }

    pub fn to_row(&self) -> Row {
        Row {
            id: self.id.clone(),
            name: self.name.clone(),
            email: self.email.clone()
        }
    }
}

impl Default for Row {
    fn default() -> Row {
        Row::new()
    }
}

//need to define max rows per page in constant
pub struct Page {
    number_of_rows: usize,
    row_list: [Row; 32]
}

impl Page {
    pub fn new() -> Page {
        Page {
            number_of_rows: 0,
            row_list: Default::default()
        }
    }

    pub fn add_row(&mut self, row: &Row) {
        self.row_list[self.number_of_rows] = row.to_row();
        self.number_of_rows += 1;
    }

    pub fn is_full(&self) -> bool {
        return self.number_of_rows == 32;
    }
}

//need to define max pages per table in constant
pub struct Table {
    number_of_pages: usize,
    page_list: Vec<Page>
}

impl Table {
    pub fn new() -> Table {
        let mut page_list = vec![];
        page_list.push(Page::new());

        Table {
            number_of_pages: 0,
            page_list: page_list
        }
    }

    pub fn add_row(&mut self, row: &Row) {
        if self.page_list[self.number_of_pages].is_full() {
            self.number_of_pages += 1;
            self.page_list.push(Page::new());
        }

        self.page_list[self.number_of_pages].add_row(row);
    }

    pub fn print(&self) {
        for i in 0..self.number_of_pages+1 {
            let page = &self.page_list[i];

            for j in 0..page.number_of_rows {
                println!("{} {} {}", page.row_list[j].id, page.row_list[j].name, page.row_list[j].email);
            }
        }
    }
}