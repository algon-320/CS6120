use bril_rs::*;
use cs6120::basic_block::basic_blocks;
use cs6120::dead_code_elim;

fn main() {
    let mut p = load_program();
    for f in p.functions.iter_mut() {
        let mut blocks = basic_blocks(&f.instrs);
        loop {
            let mut updated = false;
            let mut new = Vec::new();
            for b in blocks {
                let tmp = dead_code_elim::local(&b);
                if b != tmp {
                    updated = true;
                }
                new.push(tmp);
            }
            blocks = new;
            if !updated {
                break;
            }
        }
        f.instrs = blocks.into_iter().flatten().collect();
        f.instrs = dead_code_elim::global(&f.instrs);
    }
    output_program(&p);
}
