/// Define the error `Struct` feedback for each implemented error
#[derive(Debug)]
pub struct ErrorFeedback {
    /// The origin message for certain error
    pub message: String,
}

/// Default implementation of [`ErrorFeedback`]
impl ErrorFeedback {
    /// Print error message into console
    pub fn print(&self){
        let msg = &self.message;
        eprintln!("{msg:?}");
    }
}

/// When ever a given path isn't found, would [`path_not_found`] call.
/// Usual is [`path_not_found`] to check if a directory or file still exists.
/// 
/// # Parameters
/// 
/// * `path: &str` - The path what isn't found
pub fn path_not_found(path: &str) -> ErrorFeedback{
    ErrorFeedback{
        message: format!("Path '{path}' not found")
    }
}

/// Calling [`file_not_found`] has to call when explicit file source isn't found
/// 
/// # Parameters
/// 
/// * `file: &str` - The file path what isn't found.
pub fn file_not_found(file: &str) -> ErrorFeedback{
    ErrorFeedback{
        message: format!("File '{file}' not found")
    }
}

pub fn unknown_token(token: &str) -> ErrorFeedback{
    ErrorFeedback{
        message: format!("Unknown token '{token}'")
    }
}

pub fn token_not_implemented(token: &str) -> ErrorFeedback{
    ErrorFeedback{
        message: format!("Seems like the token '{token}' isn't implemented")
    }
}