use crate::cfg::{Cfg, CfgNode};
use bril_rs::*;
use std::collections::{HashMap, HashSet};

pub type AnalysisResult<Set> = HashMap<String, (Set, Set)>;

pub trait DataFlowAnalysisBase {
    type Set;

    fn transfer(node: &CfgNode, entry: Self::Set) -> Self::Set;
    fn edge(exit: &CfgNode, entry: &CfgNode, exit: Self::Set) -> Self::Set;
    fn merge(sets: Vec<Self::Set>) -> Self::Set;
}

pub trait DataFlowAnalysis: DataFlowAnalysisBase {
    fn drive(cfg: &Cfg, init: Self::Set) -> AnalysisResult<Self::Set>;
}

fn drive_forward<A>(cfg: &Cfg, init: A::Set) -> AnalysisResult<A::Set>
where
    A: DataFlowAnalysisBase,
    A::Set: Clone + Default + PartialEq,
{
    let mut result = AnalysisResult::new();
    for name in cfg.nodes.keys() {
        result.insert(name.clone(), (A::Set::default(), A::Set::default()));
    }

    let mut worklist = HashSet::new();
    worklist.insert(cfg.entry.clone());

    while !worklist.is_empty() {
        let name = worklist.iter().next().unwrap().clone();
        worklist.remove(&name);

        let node = cfg.nodes.get(&name).unwrap();
        let mut out_p: Vec<_> = node
            .prev
            .iter()
            .map(|p| {
                let s = result.get(p).unwrap().1.clone();
                A::edge(cfg.nodes.get(p).unwrap(), node, s)
            })
            .collect();
        if name == cfg.entry {
            out_p.push(init.clone());
        }

        let reach = A::merge(out_p);
        let (entry, exit) = result.get_mut(&name).unwrap();
        *entry = reach.clone();
        let reach = A::transfer(&node, reach);
        if exit != &reach {
            *exit = reach;
            for nx in &node.next {
                worklist.insert(nx.clone());
            }
        }
    }
    result
}

#[allow(dead_code, unused_variables)]
fn drive_backward<A: DataFlowAnalysisBase>(cfg: &Cfg, init: A::Set) -> AnalysisResult<A::Set> {
    todo!()
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct InstructionId(String, usize);

pub struct ReachingDefinition(());

impl DataFlowAnalysisBase for ReachingDefinition {
    type Set = HashMap<String, HashSet<InstructionId>>;

    fn transfer(node: &CfgNode, mut reaching_set: Self::Set) -> Self::Set {
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

    fn edge(_exit: &CfgNode, _entry: &CfgNode, exit: Self::Set) -> Self::Set {
        exit
    }

    fn merge(sets: Vec<Self::Set>) -> Self::Set {
        let mut u = Self::Set::new();
        for s in sets {
            for (name, instrs) in s {
                let tmp = u.entry(name).or_insert_with(HashSet::new);
                tmp.extend(instrs);
            }
        }
        u
    }
}

impl DataFlowAnalysis for ReachingDefinition {
    fn drive(cfg: &Cfg, init: Self::Set) -> AnalysisResult<Self::Set> {
        drive_forward::<Self>(cfg, init)
    }
}
