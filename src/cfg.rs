use bril_rs::*;
use std::collections::HashMap;

use crate::basic_block::is_terminator;

fn extract_label(block: &[Code]) -> (Option<String>, &[Code]) {
    assert!(!block.is_empty());
    match &block[0] {
        Code::Label { label } => (Some(label.clone()), &block[1..]),
        _ => (None, block),
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CfgNode {
    pub name: String,
    pub block: Vec<Code>,
    pub next: Vec<String>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Cfg {
    pub nodes: HashMap<String, CfgNode>,
}
impl Cfg {
    #[allow(dead_code)]
    pub fn build(blocks: &[Vec<Code>]) -> Cfg {
        let mut nodes: HashMap<String, CfgNode> = HashMap::new();

        let mut pred_name = None;
        for b in blocks.iter().filter(|b| !b.is_empty()) {
            let (name, b) = extract_label(b);
            let name = name.unwrap_or_else(|| format!("bb{}", nodes.len()));

            let next = b
                .iter()
                .filter_map(|code| match code {
                    Code::Instruction(Instruction::Effect { labels, .. }) => Some(labels),
                    _ => None,
                })
                .flatten()
                .cloned()
                .collect();

            // update predecessor's "next"
            if let Some(pred) = pred_name
                .take()
                .and_then(|pred_name| nodes.get_mut(&pred_name))
            {
                pred.next.push(name.clone());
            }

            nodes.insert(
                name.clone(),
                CfgNode {
                    name: name.clone(),
                    block: b.to_vec(),
                    next,
                },
            );

            pred_name = match b.last() {
                Some(Code::Instruction(ins)) if is_terminator(ins) => None,
                _ => Some(name),
            };
        }

        Cfg { nodes }
    }
}
