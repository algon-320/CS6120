use bril_rs::*;
use std::collections::{HashMap, HashSet};

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
    pub prev: HashSet<String>,
    pub next: HashSet<String>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Cfg {
    pub entry: String,
    pub nodes: HashMap<String, CfgNode>,
}
impl Cfg {
    #[allow(dead_code)]
    pub fn build(blocks: &[Vec<Code>]) -> Cfg {
        let mut nodes: HashMap<String, CfgNode> = HashMap::new();
        let mut entry: Option<String> = None;

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
                pred.next.insert(name.clone());
            }

            nodes.insert(
                name.clone(),
                CfgNode {
                    name: name.clone(),
                    block: b.to_vec(),
                    next,
                    prev: HashSet::new(),
                },
            );
            if entry.is_none() {
                entry = Some(name.clone());
            }

            pred_name = match b.last() {
                Some(Code::Instruction(ins)) if is_terminator(ins) => None,
                _ => Some(name),
            };
        }

        let mut cfg = Cfg {
            entry: entry.expect("empty cfg"),
            nodes,
        };
        cfg.refresh_prev();
        cfg
    }

    fn refresh_prev(&mut self) {
        let mut prev_map: HashMap<String, HashSet<String>> = HashMap::new();
        for (name, node) in self.nodes.iter() {
            for nx in node.next.iter() {
                if !prev_map.contains_key(nx) {
                    prev_map.insert(nx.to_owned(), HashSet::new());
                }
                let prev = prev_map.get_mut(nx).unwrap();
                prev.insert(name.to_owned());
            }
        }
        for (name, prev) in prev_map {
            let node = self.nodes.get_mut(&name).unwrap();
            node.prev = prev;
        }
    }
}
