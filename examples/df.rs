use bril_rs::*;
use cs6120::basic_block;
use cs6120::cfg;
use cs6120::data_flow_framework::{DataFlowAnalysis, DataFlowAnalysisBase, ReachingDefinition};

use std::collections::HashSet;

fn main() {
    let p = load_program();
    for func in p.functions {
        let cfg = cfg::Cfg::build(&basic_block::basic_blocks(&func.instrs));
        let mut args = <ReachingDefinition as DataFlowAnalysisBase>::Set::new();

        for arg in func.args.iter() {
            args.insert(arg.name.clone(), HashSet::new());
        }
        let result = ReachingDefinition::drive(&cfg, args);
        for name in cfg.nodes.keys() {
            let (ent, out) = result.get(name).unwrap();
            println!("{}:", name);
            println!("  in: {:?}", ent);
            println!(" out: {:?}", out);
        }
    }
}
