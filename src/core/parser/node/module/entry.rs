use crate::bin::constants;
use crate::core::applit::entities::bundle::AppLitAst;
use crate::core::feedback::error::Cause;
use crate::core::parser::node::module::statement_parser::{parse_module_statements, AstModuleNode};
use crate::core::parser::node::{AstNode, TreeBuilder};
use crate::core::parser::statements::import::ImportStatement;
use crate::core::tokenizer::lib::string_utils::literal_to_cleaned_string;
use crate::core::tokenizer::tokenize_file;
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

impl<'a> TreeBuilder<'a> {
    pub fn parse_modules(&mut self, import_statements: Vec<ImportStatement>) -> Result<(), Cause> {
        if import_statements.is_empty() {
            return Ok(());
        }

        let (sender, receiver) = unbounded::<Result<usize, Cause>>();
        let arc_ast = self.app_lit.clone_ast().unwrap();
        let arc_sender = Arc::new(Mutex::new(sender));

        self.tokenize_module_import_source(&arc_ast, &arc_sender, &receiver, import_statements);

        match receiver.try_recv() {
            Ok(message) => match message {
                Ok(index) => {
                    let import_statements = self.get_module_import_statements(index)?;

                    Ok(self.parse_modules(import_statements)?)
                }
                Err(error_cause) => {
                    Err(error_cause)
                },
            },
            Err(err) => {
                Err(Cause::UnexpectedChannelError("For parse module".into()))
            },
        }
    }

    fn tokenize_module_import_source(
        &mut self,
        arc_ast: &Arc<Mutex<AppLitAst>>,
        arc_sender: &Arc<Mutex<Sender<Result<usize, Cause>>>>,
        arc_receiver: &Receiver<Result<usize, Cause>>,
        mut import_statements: Vec<ImportStatement>,
    ) {
        let pool_amount = match constants::USE_CPU_AMOUNT_AS_THREAD_POOL {
            true => num_cpus::get(),
            false => constants::MAX_THREAD_POOLS,
        };
        let pool = ThreadPool::new(pool_amount);

        while let Some(import_statement) = import_statements.pop() {
            let arc_ast = Arc::clone(arc_ast);
            let arc_sender = Arc::clone(arc_sender);

            if let Ok(Err(error_cause)) = arc_receiver.try_recv() {
                break;
            }

            let path = literal_to_cleaned_string(&import_statement.reference.token);
            let location = self.app_lit.get_joined_location(&path);
            let module_path = self.app_lit.get_module_path(&location);

            if self.app_lit.exist_ast_node_item(&path) {
                continue;
            }

            pool.execute(move || {
                let sender = arc_sender.lock().unwrap();

                match tokenize_file(&module_path) { 
                    Ok(mut tokens) => {
                        match parse_module_statements(&mut tokens) {
                            Ok(ast_node) => {
                                let mut ast = arc_ast.lock().unwrap();
                                let index = ast.push_ast_node(ast_node);
                                ast.insert_reference(&path, index);
                                let _ = sender.send(Ok(index));
                            }
                            Err(error_cause) => {
                                let _ = sender.send(Err(error_cause));
                            }
                        }
                    },
                    Err(error_cause) => {
                        let _ = sender.send(Err(error_cause));
                    }
                }
                
            });
        }

        pool.join();
    }

    fn get_module_import_statements(&mut self, index: usize) -> Result<Vec<ImportStatement>, Cause> {
        Ok(
            if let Some(AstNode::Module(AstModuleNode::Statements(statements))) =
                self.app_lit.get_ast()?.nodes.get(index)
            {
                statements
                    .iter()
                    .filter_map(|stmt| {
                        if let AstModuleNode::Import(import_statement) = stmt {
                            return Some(import_statement.clone());
                        }
                        None
                    })
                    .collect()
            } else {
                vec![]
            },
        )
    }
}
