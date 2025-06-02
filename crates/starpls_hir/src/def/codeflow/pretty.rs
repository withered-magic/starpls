use std::fmt::Write;

use crate::def::codeflow::CodeFlowGraph;
use crate::def::codeflow::FlowNode;
use crate::def::codeflow::FlowNodeId;

macro_rules! w {
    ($dst:expr, $($arg:tt)*) => {
        { let _ = write!($dst, $($arg)*); }
    };
}

macro_rules! wln {
    ($dst:expr) => {
        { let _ = writeln!($dst); }
    };
    ($dst:expr, $($arg:tt)*) => {
        { let _ = writeln!($dst, $($arg)*); }
    };
}

impl CodeFlowGraph {
    pub(crate) fn pretty_print(&self) -> String {
        CodeFlowGraphPrettyCtx {
            cfg: self,
            result: String::new(),
            indent: String::new(),
        }
        .pretty_print()
    }
}

struct CodeFlowGraphPrettyCtx<'a> {
    cfg: &'a CodeFlowGraph,
    result: String,
    indent: String,
}

impl CodeFlowGraphPrettyCtx<'_> {
    fn pretty_print(mut self) -> String {
        wln!(&mut self.result, "def main():");
        self.push_indent_level();
        self.format_flow_nodes();
        self.result
    }

    fn format_flow_nodes(&mut self) {
        for (id, flow_node) in &self.cfg.flow_nodes {
            let formatted_id = self.format_flow_node_id(id);
            wln!(&mut self.result, "{}{}: {{", self.indent, formatted_id);
            self.push_indent_level();
            wln!(&mut self.result, "{}data: {:?}", self.indent, flow_node);
            w!(&mut self.result, "{}antecedents: [", self.indent);
            match flow_node {
                FlowNode::Assign { antecedent, .. } => {
                    self.result.push_str(&self.format_flow_node_id(*antecedent));
                }
                FlowNode::Branch { antecedents } | FlowNode::Loop { antecedents } => {
                    for (i, antecedent) in antecedents.iter().enumerate() {
                        if i > 0 {
                            self.result.push_str(", ");
                        }
                        self.result.push_str(&self.format_flow_node_id(*antecedent));
                    }
                }
                _ => {}
            }

            self.result.push_str("]\n");
            self.pop_indent_level();
            wln!(&mut self.result, "{}}}", self.indent);
            self.result.push('\n');
        }
    }

    fn format_flow_node_id(&self, id: FlowNodeId) -> String {
        format!("'bb{}", id.index())
    }

    fn push_indent_level(&mut self) {
        self.indent.push_str("    ");
    }

    fn pop_indent_level(&mut self) {
        for _ in 0..4 {
            self.indent.pop();
        }
    }
}
