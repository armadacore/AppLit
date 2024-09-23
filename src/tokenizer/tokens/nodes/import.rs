use crate::tokenizer::tokens::Token;

#[derive(Debug)]
pub struct ImportDeclaration {
    pub specifier: Vec<String>,
    pub from: Option<String>,
}

pub fn check(ast: &mut Vec<Token>, token: &str) -> bool {
    if token == "import" {
        ast.push(Token::Import(ImportDeclaration{
            specifier: vec![],
            from: None
        }));
        
        return true;
    }
    
    let last_declaration_item = ast.last_mut();
    if let Some(Token::Import(declaration)) = last_declaration_item{
        let mut current_token= String::from(token);
        
        if current_token.starts_with("{"){
            let current_token = current_token.remove(0).to_string();
        }
        
        if current_token.ends_with(","){
            current_token.pop();
        }

        if current_token.ends_with("}"){
            current_token.pop();
        }
        
        if current_token.ne("from"){
            declaration.specifier.push(current_token);
        }
        
        return true;
    }
    
    false
}