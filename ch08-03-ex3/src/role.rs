use crate::department::Department;
use crate::department::DepartmentError;
use std::str::FromStr;
use thiserror::Error;

pub struct Role {
    pub employee: String,
    pub dept: Department,
}

impl FromStr for Role {
    type Err = RoleError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split(" ").collect();
        let last_word = words[3..].join(" ");
        let mut words = words[..3].to_vec();
        words.push(&last_word);
        match words[..] {
            ["ADD", employee, "TO", department] => Ok(Role {
                employee: employee.to_string(),
                dept: department.parse()?,
            }),
            _ => Err(RoleError::ParseStr(s.to_string())),
        }
    }
}

#[derive(Error, Debug)]
pub enum RoleError {
    #[error("Cannot parse string {0} into Role. Make sure to use ADD and TO.")]
    ParseStr(String),
    #[error("{0}")]
    DepartmentError(#[from] DepartmentError),
}
