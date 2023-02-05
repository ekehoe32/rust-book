use convert_case::{Case, Casing};
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use thiserror::Error;

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
// enum
pub enum Department {
    Engineering,
    Sales,
    Manufacturing,
    HumanResources,
    Marketing,
}

impl Department {
    pub fn name(&self) -> String {
        return format!("{self:?}").to_case(Case::Title);
    }
    pub fn variants_pretty() -> String {
        return Department::iter()
            .map(|x| x.name())
            .collect::<Vec<String>>()
            .join(", ");
    }
}

impl FromStr for Department {
    type Err = DepartmentError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Engineering" => Ok(Department::Engineering),
            "Sales" => Ok(Department::Sales),
            "Manufacturing" => Ok(Department::Manufacturing),
            "Human Resources" => Ok(Department::HumanResources),
            "Marketing" => Ok(Department::Marketing),
            _ => Err(DepartmentError::ParseStr(s.to_string())),
        }
    }
}

#[derive(Error, Debug)]
pub enum DepartmentError {
    #[error(
        "Cannot parse string {0} into Department. Make sure it is one of {}.",
        Department::variants_pretty()
    )]
    ParseStr(String),
}
