use std::rc::Rc;
use crate::core::tokenizer::reader::{TokenReaderSnapshot};
use super::DeclarationValidation;

pub fn validate<F>(declaration: &mut DeclarationValidation, token: &str, mut callback: F)
where
    F: 'static + FnMut(TokenReaderSnapshot),
{
    let token_ref = Rc::new(token.to_string());
    let mut can_skip = false;

    declaration.stack.push(Box::new(move |snapshot|{
        let token_clone = Rc::clone(&token_ref);
        
        if can_skip {
            if is_equal(&snapshot, &token_clone) {
                return false
            }
            
            return true;
        }

        if let Some(snapshot_token) = &snapshot.token {
            if is_equal(&snapshot, &token_clone) {
                can_skip = true;
                callback(snapshot);
            }
        }

        can_skip
    }));
}

fn is_equal(snapshot: &TokenReaderSnapshot, token_ref: &Rc<String>) -> bool {
    if let Some(snapshot_token) = &snapshot.token {
        if snapshot_token.eq(token_ref.as_str()) {
            return true;
        }
    }
    
    false
}