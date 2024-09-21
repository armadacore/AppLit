#[derive(Debug)]
pub struct ErrorFeedback {
    pub message: String,
}

impl  ErrorFeedback {
    pub fn print(&self){
        let msg = &self.message;
        eprintln!("{msg:?}");
    }
}

pub fn path_not_found(path: &str) -> ErrorFeedback{
    ErrorFeedback{
        message: format!("Path '{path}' not found")
    }
}

pub fn file_not_found(file: &str) -> ErrorFeedback{
    ErrorFeedback{
        message: format!("File '{file}' not found")
    }
}