use bril_rs::*;
use cs6120::basic_block;
use cs6120::cfg;

fn main() {
    let p = load_program();
    for func in p.functions {
        println!("digraph {} {{", func.name);
        println!("  node [shape=box];");
        let cfg = cfg::Cfg::build(&basic_block::basic_blocks(&func.instrs));
        let mut names: Vec<String> = cfg.nodes.keys().cloned().collect();
        names.sort();
        for name in &names {
            println!("  {};", name);
        }
        for name in names {
            let node = cfg.nodes.get(&name).unwrap();
            let next = node.next.join(",");
            println!("  {} -> {{ {} }}", name, next);
        }
        println!("}}");
    }
}
