
use std::collections::HashMap;

pub struct Company {
    departments: HashMap<String, Vec<Employee>>
}

impl Company {
    pub fn new() -> Self {
        Self {
            departments: HashMap::new(),
        }
    }

    pub fn parse(&mut self, parts: Vec<&str>) {
        match &parts[..] {
            ["add", employee, "to", department] => {
                println!("You want to add '{}' to '{}'", employee, department);
                self.add_employee(
                    department.to_string(),
                    employee.to_string(),
                );
            },
            ["list", "employees"] => {
                println!("You want to list all employees by department");
                match self.get_departments() {
                    Some(departments) => {
                        for d in departments {
                            println!("Department: {}", d);
                            println!("Employees: {:?}", self.get_employees(&d));
                        }
                    },
                    // do nothing
                    None => println!("No employees to list."),
                }
                // TODO
            },
            ["list", "department", department] => {
                println!("You want to list all employees in department '{}'", department);
                println!("Employees: {:?}", self.get_employees(&department));
            },
            _ => {
                println!("Command not recognized: {:?}", parts);
            },
        }
    }

    pub fn add_employee(&mut self, department_name: String, employee_name: String) {
        let department = self.departments
                .entry(department_name)
                .or_insert(vec![]);
        department.push(Employee::new(employee_name))
    }

    pub fn get_departments(&self) -> Option<Vec<String>> {
        if self.departments.is_empty() {
            return None;
        }
        let mut result: Vec<String> = self.departments.keys().cloned().collect();
        result.sort_unstable();
        Some(result)
    }

    pub fn get_employees(&self, department_name: &str) -> Option<&[Employee]> {
        match self.departments.get(department_name) {
            None => None,
            Some(v) => Some(&v[..]),
        }
    }
}

#[derive(Debug)]
struct Employee {
    name: String,
}

impl Employee {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

