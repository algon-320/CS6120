use bril_rs::*;

type BasicBlock = Vec<Code>;

pub fn is_terminator(ins: &Instruction) -> bool {
    match ins {
        Instruction::Effect { op, .. } => {
            matches!(op, EffectOps::Jump | EffectOps::Branch | EffectOps::Return)
        }
        _ => false,
    }
}

#[allow(dead_code)]
pub fn basic_blocks(instrs: &[Code]) -> Vec<BasicBlock> {
    let mut blocks: Vec<BasicBlock> = Vec::new();
    let mut cur = BasicBlock::new();
    for code in instrs {
        match code {
            Code::Label { .. } => {
                if !cur.is_empty() {
                    blocks.push(cur);
                    cur = BasicBlock::new();
                }
                cur.push(code.clone());
            }
            Code::Instruction(ins) => {
                cur.push(code.clone());
                if is_terminator(ins) {
                    blocks.push(cur);
                    cur = BasicBlock::new();
                }
            }
        }
    }
    if !cur.is_empty() {
        blocks.push(cur);
    }
    blocks
}
