use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Row {
    pub id: String,
    name: String,
    email: String
}

impl Row {
    pub fn new() -> Row {
        Row {
            id: String::new(),
            name: String::new(),
            email: String::new()
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
}

impl Default for Row {
    fn default() -> Row {
        Row::new()
    }
}
