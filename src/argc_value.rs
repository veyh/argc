use crate::utils::hyphens_to_underscores;

pub const VARIABLE_PREFIX: &str = "argc";

#[derive(Debug, PartialEq, Eq)]
pub enum ArgcValue {
    Single(String, String),
    Multiple(String, Vec<String>),
    PositionalSingle(String, String),
    PositionalMultiple(String, Vec<String>),
    FnName(String),
}

impl ArgcValue {
    pub fn to_shell(values: Vec<Self>) -> String {
        let mut variables = vec![];
        let mut positional_args = vec![];
        for value in values {
            match value {
                ArgcValue::Single(name, value) => {
                    variables.push(format!(
                        "{}_{}={}",
                        VARIABLE_PREFIX,
                        hyphens_to_underscores(&name),
                        value
                    ));
                }
                ArgcValue::Multiple(name, values) => {
                    variables.push(format!(
                        "{}_{}=( {} )",
                        VARIABLE_PREFIX,
                        name,
                        values.join(" ")
                    ));
                }
                ArgcValue::PositionalSingle(name, value) => {
                    variables.push(format!(
                        "{}_{}={}",
                        VARIABLE_PREFIX,
                        hyphens_to_underscores(&name),
                        &value
                    ));
                    positional_args.push(value.to_string());
                }
                ArgcValue::PositionalMultiple(name, values) => {
                    variables.push(format!(
                        "{}_{}=( {} )",
                        VARIABLE_PREFIX,
                        hyphens_to_underscores(&name),
                        values.join(" ")
                    ));
                    positional_args.extend(values);
                }
                ArgcValue::FnName(name) => {
                    if positional_args.is_empty() {
                        variables.push(name.to_string());
                    } else {
                        variables.push(format!("{} {}", name, positional_args.join(" ")));
                    }
                }
            }
        }
        variables.join("\n")
    }
}
