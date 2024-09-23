use crate::tokenizer::tokens::Token;
use crate::tokenizer::tokens::verification::TokenToAst;

#[derive(Debug, Clone)]
pub struct ImportDeclaration {
    pub specifier: Vec<String>,
    pub from: Option<String>,
}

pub fn check(t2a: &mut TokenToAst) -> bool {
    if t2a.token.is_none() {
        return false;
    }
    
    if let Some(token) = t2a.current_token(){
        if token == "import"{
            let mut declaration = ImportDeclaration{
                specifier: vec![],
                from: None
            };

            while let Some(token) = t2a.next() {
                let mut current_token = String::from(token);
                
                if current_token == "{"{
                    continue;
                }
                
                if current_token == "}"{
                    continue;
                }
                
                if current_token == "from"{
                    if let Some(from) = t2a.next(){
                        let cleaned = from.replace("'", "").replace(";", "");
                        declaration.from = Some(cleaned);   
                    }
                    break;
                }
                
                if current_token.starts_with("{"){
                    current_token.remove(0).to_string();
                }

                if current_token.ends_with(","){
                    current_token.pop();
                }

                if current_token.ends_with("}"){
                    current_token.pop();
                }

                declaration.specifier.push(current_token);
            }

            t2a.ast.push(Token::Import(declaration));
            
            return true;
        }
    }
    
    false
}