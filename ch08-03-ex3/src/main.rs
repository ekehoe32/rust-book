mod action;
mod department;
mod role;
use crate::action::DeptAction;
use crate::department::Department;
use crate::role::Role;
use std::{collections::HashMap, io};

fn main() {
    // init employees map
    let mut employee_depts = HashMap::new();

    loop {
        // give user options
        println!(
            "\nSelect one:\n
            \t(a) Add employee to department\n
            \t(d) Get list of employees in a department\n
            \t(p) Get list of all people by department\n
            \t(q) Quit\n"
        );

        // get response
        let mut response = String::new();
        match io::stdin().read_line(&mut response) {
            Ok(_) => (),
            Err(_) => {
                println!("Error reading response, please try again!");
                continue;
            }
        }

        // process response
        let action: DeptAction = match response.trim().parse() {
            Ok(action) => action,
            Err(e) => {
                eprintln!("{e}");
                continue;
            }
        };

        // process actions
        match action {
            DeptAction::AddEmployee => {
                // get response
                println!("\nEnter an ADD statement, i.e., ADD [Employee] TO [Department]");
                println!(
                    "Department can be one of {}.\n",
                    Department::variants_pretty()
                );
                response.clear();
                match io::stdin().read_line(&mut response) {
                    Ok(_) => (),
                    Err(_) => {
                        println!("Error reading response, please try again!");
                        continue;
                    }
                };

                // process the response
                let role: Role = match response.trim().parse() {
                    Ok(role) => role,
                    Err(err) => {
                        eprintln!("{err} Try Again!");
                        continue;
                    }
                };
                match employee_depts.insert(role.employee[..].to_string(), role.dept) {
                    Some(_) => println!(
                        "Successfully changed {}'s deparment to {:?}!",
                        role.employee, role.dept
                    ),
                    None => println!("Successfully added {} to {:?}!", role.employee, role.dept),
                }
            }
            DeptAction::GetEmployeesInDept => {
                // get response
                response.clear();
                println!("Enter a department name:");
                match io::stdin().read_line(&mut response) {
                    Ok(_) => (),
                    Err(_) => {
                        println!("Error reading response, please try again!");
                        continue;
                    }
                };

                // parse the response
                let dept: Department = match response.trim().parse() {
                    Ok(dept) => dept,
                    Err(err) => {
                        eprintln!("{err} Try again.");
                        continue;
                    }
                };

                // get the employees in the department in the sort
                let mut employees: Vec<String> = employee_depts
                    .iter()
                    .filter(|(_, v)| **v == dept)
                    .map(|(k, _)| k.to_string())
                    .collect();
                employees.sort();

                // print to console
                println!(
                    "\nThe employees in {dept:?} are:\n\n\t- {}",
                    employees.join("\n\t- ")
                )
            }
            DeptAction::GetAllEmployeesByDept => {
                // sort the employees by name and department then sort
                let mut employee_depts_vec: Vec<(String, Department)> =
                    employee_depts.to_owned().into_iter().collect();
                employee_depts_vec.sort_by(|(a, b), (c, d)| (b, a).cmp(&(d, c)));

                // print employees to console
                println!(
                    "\nThe employees in are:\n\n\t- {}",
                    employee_depts_vec
                        .iter()
                        .map(|(e, d)| format!("{e} -> {d:?}"))
                        .collect::<Vec<String>>()
                        .join("\n\t- ")
                )
            }
            DeptAction::Quit => break,
        }
    }
}
