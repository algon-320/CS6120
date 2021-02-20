use bril_rs::*;
use cs6120::basic_block::basic_blocks;
use cs6120::lvn;

fn main() {
    let mut p = load_program();
    for f in p.functions.iter_mut() {
        let blocks = basic_blocks(&f.instrs);
        f.instrs = blocks
            .into_iter()
            .flat_map(|mut b| {
                lvn::local_value_numbering(&mut b);
                b
            })
            .collect();
    }
    output_program(&p);
}
