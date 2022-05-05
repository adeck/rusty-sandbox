
use company::Company;

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

