use bril_rs::*;
use std::collections::{HashMap, HashSet};

pub fn local(block: &[Code]) -> Vec<Code> {
    let mut instrs: Vec<Option<Code>> = block.iter().cloned().map(Some).collect();
    let mut assign = HashMap::new();

    for code in instrs.iter_mut() {
        let ins = match code.as_ref().unwrap() {
            Code::Label { .. } => continue,
            Code::Instruction(ins) => ins,
        };
        if let Instruction::Value { args, .. } | Instruction::Effect { args, .. } = ins {
            for arg in args {
                assign.remove(arg);
            }
        }
        if let Instruction::Value { dest, .. } | Instruction::Constant { dest, .. } = ins {
            if let Some(old) = assign.insert(dest.clone(), code) {
                old.take();
            }
        }
    }

    instrs
        .into_iter()
        .filter_map(std::convert::identity)
        .collect()
}

pub fn global(instrs: &[Code]) -> Vec<Code> {
    let used: HashSet<_> = instrs
        .iter()
        .filter_map(|code| match code {
            Code::Instruction(Instruction::Value { args, .. })
            | Code::Instruction(Instruction::Effect { args, .. }) => Some(args),
            _ => None,
        })
        .flatten()
        .collect();
    instrs
        .iter()
        .filter(|code| match code {
            Code::Instruction(Instruction::Constant { dest, .. })
            | Code::Instruction(Instruction::Value { dest, .. }) => used.contains(dest),
            _ => true,
        })
        .cloned()
        .collect()
}
