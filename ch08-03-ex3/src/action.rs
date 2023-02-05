use std::str::FromStr;
use thiserror::Error;

pub enum DeptAction {
    AddEmployee,
    GetEmployeesInDept,
    GetAllEmployeesByDept,
    Quit,
}

impl FromStr for DeptAction {
    type Err = ActionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(DeptAction::AddEmployee),
            "d" => Ok(DeptAction::GetEmployeesInDept),
            "p" => Ok(DeptAction::GetAllEmployeesByDept),
            "q" => Ok(DeptAction::Quit),
            _ => Err(ActionError::ParseDeptAction(s.to_string())),
        }
    }
}

#[derive(Debug, Error)]
pub enum ActionError {
    #[error("Cannot parse your action: {0}")]
    ParseDeptAction(String),
}
