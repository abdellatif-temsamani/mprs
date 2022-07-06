use std::env::Args;

/// # Type
/// types for flag
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Name,
    Command,
    Config,
    Error,
}

/// # Flag
///
/// Flag struct describe each **individual** flag
/// composed by:
/// - __order__ : usize -> order of the flag
/// - __flag__ : String -> value of flag
/// - __type__ : Type -> type of flag
///
#[derive(Debug, Clone)]
pub struct Flag {
    pub order: usize,
    pub value: String,
    pub flag_type: Type,
}

impl Flag {
    /// # new flag
    ///
    /// Create a new flag with order returns Flag
    ///
    /// ## Params
    /// - **order**: usize
    /// - **args**: &mut Args
    ///
    pub fn new(order: usize, args: &mut Args) -> Self {
        match args.nth(0) {
            Some(value) => Flag {
                order,
                value: value.clone(),
                flag_type: if value.starts_with("-") {
                    Type::Config
                } else if order == 0 {
                    Type::Name
                } else {
                    Type::Command
                },
            },

            None => Flag {
                order,
                value: "[Error] -> None".to_owned(),
                flag_type: Type::Error,
            },
        }
    }
}
