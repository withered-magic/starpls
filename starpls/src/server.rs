use dashmap::{mapref::entry::Entry, DashMap};
use lsp_server::{Connection, ReqQueue};
use tree_sitter::{InputEdit, Parser, Tree};

use crate::{document::DocumentManager, utils::Edit};

pub(crate) struct Reparser {
    parser: Parser,
    tree: Option<Tree>,
}

impl Reparser {
    pub(crate) fn new(parser: Parser) -> Self {
        Self { parser, tree: None }
    }

    pub(crate) fn reparse(&mut self, input: &str, edits: Option<Vec<InputEdit>>) -> Option<Tree> {
        // If edits have been specified, apply them to the old tree and use the old tree during reparsing.
        // Otherwise, discard the old tree and parse from scratch.
        let mut old_tree: Option<&Tree> = None;
        if let Some(edits) = edits {
            if let Some(old_tree) = self.tree.as_mut() {
                for edit in edits {
                    old_tree.edit(&edit);
                }
            }
            old_tree = self.tree.as_ref();
        }

        self.tree = self.parser.parse(input, old_tree);
        self.tree.as_ref().cloned()
    }
}

pub(crate) struct Server {
    pub(crate) connection: Connection,
    pub(crate) req_queue: ReqQueue<(), ()>,
    pub(crate) document_manager: DocumentManager,
    pub(crate) reparsers: DashMap<u32, Reparser>,
}

impl Server {
    pub(crate) fn new(connection: Connection) -> anyhow::Result<Self> {
        Ok(Server {
            connection,
            req_queue: Default::default(),
            document_manager: Default::default(),
            reparsers: Default::default(),
        })
    }

    pub(crate) fn process_changes(&mut self) {
        let (_has_opened_or_closed_documents, changed_documents) =
            self.document_manager.take_changes();

        for (file_id, edit) in changed_documents {
            self.reparse(file_id, edit);
        }
    }

    pub(crate) fn reparse(&self, file_id: u32, edit: Edit) {
        eprintln!("reparse");
        let mut reparser = self.reparsers.entry(file_id).or_insert_with(|| {
            let mut parser = Parser::new();
            parser
                .set_language(tree_sitter_starlark::language())
                .unwrap();
            Reparser::new(parser)
        });

        let input = self.document_manager.contents(file_id).unwrap_or("");
        let edits = match edit {
            Edit::Incremental(edits) => Some(edits),
            Edit::Full => None,
        };

        if let Some(tree) = reparser.reparse(input, edits) {
            eprintln!("sexp for tree: {}", tree.root_node().to_sexp());
        }
    }
}

// Parser
// reparse(), takes input string + edits
// if vector of edits isn't specified, reparse without reusing tree
// otherwise, apply edits and reparse with old tree

// pub(crate) struct Parser {
//     tree:
// }
