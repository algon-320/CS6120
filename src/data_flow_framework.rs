use crate::cfg::{Cfg, CfgNode};
use bril_rs::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct InstructionId(String, usize);

pub type Set = HashMap<String, HashSet<InstructionId>>;

fn transfer(node: &CfgNode, mut reaching_set: Set) -> Set {
    for (i, code) in node.block.iter().enumerate() {
        let ins = match code {
            Code::Label { .. } => continue,
            Code::Instruction(ins) => ins,
        };
        match ins {
            Instruction::Constant { dest, .. } | Instruction::Value { dest, .. } => {
                let mut instrs = HashSet::new();
                instrs.insert(InstructionId(node.name.clone(), i));
                reaching_set.insert(dest.clone(), instrs);
            }
            _ => {}
        }
    }
    reaching_set
}

fn merge(sets: Vec<Set>) -> Set {
    let mut u = Set::new();
    for s in sets {
        for (name, instrs) in s {
            let tmp = u.entry(name).or_insert_with(HashSet::new);
            tmp.extend(instrs);
        }
    }
    u
}

pub fn reaching_definition(cfg: &Cfg, init: Set) {
    let mut ent: HashMap<String, Set> = HashMap::new();
    let mut out: HashMap<String, Set> = HashMap::new();

    let mut worklist = HashSet::new();
    worklist.insert(cfg.entry.clone());

    while !worklist.is_empty() {
        let name = worklist.iter().next().unwrap().clone();
        worklist.remove(&name);
        let node = cfg.nodes.get(&name).unwrap();

        let out_p: Vec<_> = node
            .prev
            .iter()
            .map(|p| out.entry(p.clone()).or_insert_with(Set::new).clone())
            .collect();
        let reach = if name == cfg.entry {
            init.clone()
        } else {
            merge(out_p)
        };
        ent.insert(name.clone(), reach.clone()); // update
        let reach = transfer(&node, reach);
        if out.get(&name) != Some(&reach) {
            out.insert(name.clone(), reach); // update
            for nx in &node.next {
                worklist.insert(nx.clone());
            }
        }
    }

    for name in cfg.nodes.keys() {
        println!("{}:", name);
        println!("  in: {:?}", ent.get(name));
        println!(" out: {:?}", out.get(name));
    }
}
