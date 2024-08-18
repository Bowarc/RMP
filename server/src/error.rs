#[derive(Debug, thiserror::Error)]
pub enum Error{
    #[error("This is a test error and shouldn't be used in any public context")]
    Test
}


#[derive(Debug, thiserror::Error)]
pub enum PlayerError{
    #[error("This is a test error and shouldn't be used in any public context")]
    Test,

    #[error("The current stack is empty")]
    EmptyStack,

    #[error("{name} player failled to initialize due to: {error}")]
    Initialisation{
        name: String,
        error: String
    },

    #[error("Could not read file: {target} due to: {error}")]
    FileRead{
        target: String,
        error: String
    },

}