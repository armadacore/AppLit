use crate::core::tokenizer::entities::snapshot::TokenSnapshot;
use std::fmt;

#[derive(Debug, Clone)]
pub enum AstError {
    UnexpectedToken(TokenSnapshot),
    UnexpectedError(Option<TokenSnapshot>),
    UnexpectedEOF,
}

impl fmt::Display for AstError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AstError::UnexpectedToken(snapshot) => {
                write!(
                    f,
                    "Unexpected Error at {:#?} for {:#?}",
                    snapshot.location, snapshot.token
                )
            }
            AstError::UnexpectedError(snapshot) => {
                if snapshot.is_some() {
                    let snapshot = snapshot.as_ref().unwrap();
                    return write!(
                        f,
                        "Unexpected Error at {:#?} for {:#?}",
                        snapshot.location, snapshot.token
                    );
                }

                write!(f, "Unexpected Error")
            }
            AstError::UnexpectedEOF => write!(f, "Unexpected EOF"),
        }
    }
}
