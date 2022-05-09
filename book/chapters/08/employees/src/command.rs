

pub enum Command {}

impl Command {
    pub fn parse(parts: Vec<&str>) -> Option<Self> {
        if parts.is_empty() {
            return None;
        }
        match &parts[..] {
            ["add", employee, "to", department] => {
                println!("You want to add '{}' to '{}'", employee, department);
                None // TODO
            },
            ["list", "employees"] => {
                println!("You want to list all employees by department");
                None // TODO
            },
            ["list", "department", department] => {
                println!("You want to list all employees in department '{}'", department);
                None // TODO
            },
            _ => None,
        }
    }
}


