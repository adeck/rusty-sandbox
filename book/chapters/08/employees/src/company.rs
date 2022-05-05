
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

    pub fn get_employees(&self, department_name: String) -> Option<&[Employee]> {
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
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

