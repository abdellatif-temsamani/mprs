/// # Type
/// types for flag
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
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
    pub value: String,
    pub flag_type: Type,
}

impl Flag {
    /// # new flag
    ///
    /// Create a new flag with order
    ///
    /// ## Params
    /// - **order**: usize
    /// - **args**: &mut Args
    ///
    pub fn new(arg: String) -> Self {
        Self {
            value: arg.clone(),
            flag_type: if arg.starts_with('-') {
                Type::Config
            } else {
                Type::Command
            },
        }
    }
}
