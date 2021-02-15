use bril_rs::*;
use cs6120::basic_block;

fn main() {
    let p = load_program();
    for func in p.functions {
        println!("{}:", func.name);
        let blocks = basic_block::basic_blocks(&func.instrs);
        for b in blocks {
            println!("{:?}", b);
        }
    }
}
