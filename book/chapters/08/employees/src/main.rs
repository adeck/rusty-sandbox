
use std::io;
use std::collections::HashMap;

fn main() {
    let mut us = Company::new();
    'repl: loop {
        let line = read_line();
        let parts: Vec<&str> = line.split(' ').collect();
        if parts == vec![""] {
            println!("An empty line? Weak sauce.");
        }
        println!("The parts were: {:?}", parts);
        // TODO -- read user input
        // TODO -- attempt to parse user input
        // TODO -- act on relevant command
    }
}

/// Reads a line from stdin, trims and lowercases that line
fn read_line() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line properly. Exiting...");
    line.trim().to_lowercase()
}

struct Company {
    departments: HashMap<String, Vec<Employee>>
}

impl Company {
    fn new() -> Self {
        Self {
            departments: HashMap::new(),
        }
    }

    fn add_employee(&mut self, department_name: String, employee_name: String) {
        let department = self.departments
                .entry(department_name)
                .or_insert(vec![]);
        department.push(Employee::new(employee_name))
    }

    fn get_departments(&self) -> Option<Vec<String>> {
        if self.departments.is_empty() {
            return None;
        }
        let mut result: Vec<String> = self.departments.keys().cloned().collect();
        result.sort_unstable();
        Some(result)
    }

    fn get_employees(&self, department_name: String) -> Option<&[Employee]> {
        match self.departments.get(&department_name) {
            None => None,
            Some(v) => Some(&v[..]),
        }
    }
}

struct Employee {
    name: String,
}

impl Employee {
    fn new(name: String) -> Self {
        Self { name }
    }
}

fn test_unit() {
    let mut us = Company::new();
    assert_eq!(us.get_departments(), None);
    for person in [
        "Betty Saleswoman",
        "Bobby Salesman",
        "Janey Salesperson",
        "Angela the Sales Wench",
        "Brickbat the Scallawag",
    ] {
        us.add_employee("Sales".to_string(), person.to_string());
    }
    assert_eq!(us.get_departments(), Some(vec!["Sales".to_string()]));
    for person in [
        "Rikki Tikki Tavi",
        "Cooch McGee"
    ] {
        us.add_employee("Marketing".to_string(), person.to_string());
    }
    assert_eq!(us.get_departments(), Some(vec![
        "Marketing".to_string(),
        "Sales".to_string(),
    ]));
    for person in [
        "Pancake Abernathy",
        "Pancake Abernathy II",
        "Pancake Abernathy III",
        "Nepotism Abernathy",
        "Tism Abernathy",
        "PRISM Abernathy",
        "Schism AberNathy",
        "Abe R. Nathy",
    ] {
        us.add_employee("Hired Goons".to_string(), person.to_string());
    }
    assert_eq!(us.get_departments(), Some(vec![
        "Hired Goons".to_string(),
        "Marketing".to_string(),
        "Sales".to_string(),
    ]));
    println!("Departments: {:?}", us.get_departments());
}
