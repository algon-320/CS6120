use bril_rs::*;
use cs6120::basic_block;
use cs6120::cfg;
use cs6120::data_flow_framework;

use std::collections::HashSet;

fn main() {
    let p = load_program();
    for func in p.functions {
        let cfg = cfg::Cfg::build(&basic_block::basic_blocks(&func.instrs));
        let mut args = data_flow_framework::Set::new();
        for arg in func.args.iter() {
            args.insert(arg.name.clone(), HashSet::new());
        }
        data_flow_framework::reaching_definition(&cfg, args);
    }
}
