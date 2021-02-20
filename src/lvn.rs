use bril_rs::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Expr {
    UnknownVar,
    Const(String),
    Value { op: ValueOps, args: Vec<usize> },
}
impl Expr {
    fn normalize(&mut self) {
        use ValueOps::*;
        match self {
            Self::Value { op: Add, args }
            | Self::Value { op: Fadd, args }
            | Self::Value { op: Mul, args }
            | Self::Value { op: Fmul, args }
            | Self::Value { op: Eq, args }
            | Self::Value { op: Feq, args } => {
                // for commutative operator, sort its arguments.
                args.sort_unstable();
            }
            _ => { /* otherwise, do nothing. */ }
        }
    }
}

#[derive(Clone, Default)]
struct Table {
    /// variable name --> value number
    idx: HashMap<String, usize>,
    /// value number --> canonical variable name
    canon: Vec<String>,
    /// expression --> value number
    expr: HashMap<Expr, usize>,
}
impl Table {
    /// variable name --> value number
    fn index(&self, var: &str) -> Option<usize> {
        self.idx.get(var).copied()
    }
    fn index_add(&mut self, var: String, num: usize) {
        self.idx.insert(var, num);
    }

    /// value number --> canonical variable name
    fn canonical_var(&self, num: usize) -> Option<String> {
        self.canon.get(num).cloned()
    }

    /// expression --> value number
    fn find(&self, expr: &Expr) -> Option<usize> {
        let mut expr = expr.clone();
        expr.normalize();
        self.expr.get(&expr).copied()
    }

    /// register new value
    fn add(&mut self, var: String, expr: Expr) -> usize {
        if !matches!(expr, Expr::UnknownVar) {
            assert!(self.find(&expr).is_none());
        }
        match expr {
            Expr::Value {
                op: ValueOps::Id,
                args,
            } => {
                assert_eq!(args.len(), 1);
                let num = args[0];
                self.idx.insert(var, num);
                num
            }
            _ => {
                let num = self.canon.len(); // new value number
                self.canon.push(var.clone());
                self.idx.insert(var, num);
                self.expr.insert(expr, num);
                num
            }
        }
    }

    /// var --> canonical variable which has the same value as var
    fn canonicalize(&mut self, var: &str) -> String {
        self.index(var)
            .map(|num| self.canonical_var(num).unwrap())
            .unwrap_or_else(|| var.to_owned())
    }

    fn fresh_name(&self, prefix: &str) -> String {
        use rand::distributions::{Alphanumeric, Distribution};
        let mut rng = rand::thread_rng();
        loop {
            let suffix: String = Alphanumeric
                .sample_iter(&mut rng)
                .take(8)
                .map(char::from)
                .collect();
            let tmp = format!("{}.{}", prefix, suffix);
            // TODO: this may break the program.
            //       we have to ensure the name `tmp` has not been in use.
            if !self.idx.contains_key(&tmp) {
                return tmp;
            }
        }
    }
}

/// Check whether `var` will be reassigned by the instructions in `block`.
fn find_reassign(block: &[Code], var: &str) -> bool {
    for code in block.iter() {
        let ins = match code {
            Code::Label { .. } => continue,
            Code::Instruction(ins) => ins,
        };
        match ins {
            Instruction::Constant { dest, .. } | Instruction::Value { dest, .. } => {
                if dest == var {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

/// Create a new id instruction
fn id_ins(dest: String, ty: Type, var: String) -> Instruction {
    Instruction::Value {
        dest,
        op_type: ty,
        op: ValueOps::Id,
        args: vec![var],
        funcs: Vec::new(),
        labels: Vec::new(),
    }
}

/// Apply LVN
pub fn local_value_numbering(mut block: &mut [Code]) {
    let mut table = Table::default();
    while let Some((code, succ)) = block.split_first_mut() {
        let ins = match code {
            Code::Label { .. } => {
                block = succ;
                continue;
            }
            Code::Instruction(ins) => ins,
        };
        match ins {
            Instruction::Constant {
                dest,
                const_type,
                value,
                ..
            } => {
                let new_dest = if find_reassign(succ, dest) {
                    table.fresh_name(dest)
                } else {
                    dest.clone()
                };
                let value = serde_json::to_string(value).unwrap();
                let expr = Expr::Const(value);
                match table.find(&expr) {
                    None => {
                        let num = table.add(new_dest.clone(), expr);
                        if dest != &new_dest {
                            table.index_add(dest.clone(), num);
                        }
                        *dest = new_dest;
                    }
                    Some(num) => {
                        table.index_add(new_dest.clone(), num);
                        // replace with id
                        *ins = id_ins(
                            new_dest,
                            const_type.clone(),
                            table.canonical_var(num).unwrap(),
                        );
                    }
                }
            }
            Instruction::Value {
                dest,
                op_type,
                op,
                args,
                ..
            } => {
                let new_dest = if find_reassign(succ, dest) {
                    table.fresh_name(dest)
                } else {
                    dest.clone()
                };
                let mut numbered = Vec::with_capacity(args.len());
                let mut canonicalized = Vec::with_capacity(args.len());
                for arg in args.iter() {
                    let num = match table.index(arg) {
                        Some(num) => num,
                        None => table.add(arg.clone(), Expr::UnknownVar),
                    };
                    numbered.push(num);
                    canonicalized.push(table.canonicalize(arg));
                }
                let expr = Expr::Value {
                    op: *op,
                    args: numbered,
                };
                match table.find(&expr) {
                    None => {
                        let num = table.add(new_dest.clone(), expr);
                        if dest != &new_dest {
                            table.index_add(dest.clone(), num);
                        }
                        *args = canonicalized;
                        *dest = new_dest;
                    }
                    Some(num) => {
                        table.index_add(new_dest.clone(), num);
                        // replace with id
                        *ins = id_ins(new_dest, op_type.clone(), table.canonical_var(num).unwrap());
                    }
                }
            }
            Instruction::Effect { args, .. } => {
                *args = args.iter().map(|arg| table.canonicalize(arg)).collect();
            }
        }
        block = succ;
    }
}

impl std::fmt::Debug for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = self.canon.len();
        for i in 0..n {
            let cloud: Vec<_> = self
                .idx
                .iter()
                .filter_map(|(name, idx)| if *idx == i { Some(name) } else { None })
                .collect();
            let exprs: Vec<_> = self
                .expr
                .iter()
                .filter_map(|(expr, idx)| if *idx == i { Some(expr) } else { None })
                .collect();
            writeln!(
                f,
                "{:?} ---> #{} | {:?} | {}",
                cloud, i, exprs, self.canon[i]
            )?;
        }
        Ok(())
    }
}
