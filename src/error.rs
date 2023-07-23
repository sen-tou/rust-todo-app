#[derive(thiserror::Error, Debug, Clone)]
pub enum TodoError {
    #[error("Maximum number of {0} todos reached.")]
    MaximumNumberOfTodosReached(usize),
    #[error("Cannot find todo with index: {0}.")]
    TodoItemNotFound(usize),
    #[error("Some command arguments are missing. Please use h command to get help.")]
    MissingCommandArgs,
    #[error("There is no command with name: {0}. Please use h command to get help.")]
    NoCommand(String),
    #[error("Some error happened: {0}")]
    Other(String),
}
