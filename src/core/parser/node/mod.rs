use crate::bin::constants;
use crate::core::applit::entities::bundle::AppLit;
use crate::core::feedback::error::Cause;
use crate::core::parser::node::main::statement_parser::AstMainNode;
use crate::core::parser::node::module::AstModuleNode;
use crate::core::parser::statements::import::ImportStatement;
use crate::core::tokenizer::lib::string_utils::literal_to_cleaned_string;
use crossbeam_channel::unbounded;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

mod main;

pub mod module;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AstNode {
    Main(AstMainNode),
    Module(AstModuleNode),
}

pub struct TreeBuilder<'a> {
    app_lit: &'a mut AppLit,
}

impl<'a> TreeBuilder<'a> {
    pub fn new(app_lit: &'a mut AppLit) -> Self {
        TreeBuilder { app_lit }
    }

    fn parse_modules(&mut self, mut import_statements: Vec<ImportStatement>) -> Result<(), Cause> {
        let pool_amount = match constants::USE_CPU_AMOUNT_AS_THREAD_POOL {
            true => num_cpus::get(),
            false => constants::MAX_THREAD_POOLS,
        };
        let pool = ThreadPool::new(constants::MAX_THREAD_POOLS);
        let (sender, receiver) = unbounded::<Result<usize, Cause>>();
        let arc_ast = self.app_lit.clone_ast().unwrap();
        let arc_sender = Arc::new(Mutex::new(sender));

        return Ok(());

        while let Some(import_statement) = import_statements.pop() {
            let arc_ast = Arc::clone(&arc_ast);
            let arc_sender = Arc::clone(&arc_sender);

            if let Ok(Err(error_cause)) = receiver.try_recv() {
                break;
            }

            let path = literal_to_cleaned_string(&import_statement.reference.token);
            let location = self.app_lit.get_joined_location(&path);
            let module_path = self.app_lit.get_module_path(&location);

            if self.app_lit.exist_ast_node_item(&path) {
                continue;
            }

            pool.execute(move || {
                // let mut ast = arc_ast.lock().unwrap();
                // let mut tokens = tokenize_file(&module_path);
                // let sender = arc_sender.lock().unwrap();
                //
                // match parse_main_statements(&mut tokens) {
                //     Ok(ast_node) => {
                //         let index = ast.push_ast_node(ast_node);
                //         ast.insert_reference(&path, index);
                //         let _ = sender.send(Ok(index));
                //     }
                //     Err(error_cause) => {
                //         let _ = sender.send(Err(error_cause));
                //     }
                // }
            });
        }
        pool.join();

        match receiver.try_recv() {
            Ok(message) => match message {
                Ok(index) => {
                    // parse import statements
                    Ok(())
                }
                Err(error_cause) => Err(error_cause),
            },
            Err(_) => Err(Cause::UnexpectedChannelError("For parse module".into())),
        }
    }
}
