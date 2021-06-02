use std::cmp::Ordering;
use std::collections::{HashMap, LinkedList};
use std::fmt::Debug;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

/// Note: we need to represent not only integers.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Number {
    I64(i64),
    F64(f64),

    Bool(bool),
}

impl Default for Number {
    fn default() -> Self {
        Number::I64(0)
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Number::F64(fv) => {
                write!(f, "F64({})", fv)
            }
            Number::I64(iv) => {
                write!(f, "I64({})", iv)
            }
            Number::Bool(b) => {
                write!(f, "Bool({})", b)
            }
        }
    }
}

impl Number {
    pub fn from_i64(i: i64) -> Self {
        Number::I64(i)
    }

    pub fn from_f64(f: f64) -> Self {
        Number::F64(f)
    }

    pub fn from_bool(b: bool) -> Self {
        Number::Bool(b)
    }

    pub fn as_f64(self) -> f64 {
        match self {
            Number::I64(i) => i as f64,
            Number::F64(f) => f,
            Number::Bool(_) => {
                unimplemented!()
            }
        }
    }

    pub fn as_i64(self) -> i64 {
        match self {
            Number::I64(i) => i,
            Number::F64(f) => f as i64,
            Number::Bool(_) => {
                unimplemented!()
            }
        }
    }

    pub fn as_bool(self) -> bool {
        match self {
            Number::I64(i) => i != 0,
            Number::F64(f) => f != 0f64,
            Number::Bool(b) => b,
        }
    }
}

impl std::ops::Add for Number {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Number::I64(i1), Number::I64(i2)) => Number::I64(i1 + i2),
            (v1, v2) => Number::F64(v1.as_f64() + v2.as_f64()),
        }
    }
}

impl std::ops::Mul for Number {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Number::I64(i1), Number::I64(i2)) => Number::I64(i1 * i2),
            (v1, v2) => Number::F64(v1.as_f64() * v2.as_f64()),
        }
    }
}

impl std::ops::Div for Number {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            (Number::I64(i1), Number::I64(i2)) => Number::I64(i1 / i2),
            (v1, v2) => Number::F64(v1.as_f64() / v2.as_f64()),
        }
    }
}

impl std::ops::Sub for Number {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (Number::I64(i1), Number::I64(i2)) => Number::I64(i1 - i2),
            (v1, v2) => Number::F64(v1.as_f64() - v2.as_f64()),
        }
    }
}

impl std::ops::Neg for Number {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Number::I64(i) => Number::I64(-i),
            Number::F64(f) => Number::F64(-f),
            Number::Bool(_) => {
                unimplemented!()
            }
        }
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (_, Number::Bool(_)) => {
                unimplemented!()
            }
            (Number::Bool(_), _) => {
                unimplemented!()
            }
            (a, b) => a.as_f64().partial_cmp(&b.as_f64()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,

    // operations about assign and fetch
    Assign,
    Ref,

    // comparing
    Equal,
    LargerOrEqual,
    LargerThan,
    LessOrEqual,
    LessThan,
}

// #[derive(Clone, Copy, Debug, PartialEq)]
// enum BuiltinFunc {
//     Sqrt,
//     Exp,
//     Log,
//     Print,
// }

pub enum Expr {
    Number(Number),
    OneOp(Opcode, Box<Expr>),
    // Include:
    // "+" "-" "*" "/" and comparing.
    TwoOp(Opcode, Box<Expr>, Box<Expr>),
    VarRef(String),
    Assign(String, Box<Expr>),
    Flow(ControlFlow),
}

#[derive(Clone)]
pub struct ExprList(pub Option<LinkedList<Rc<Expr>>>);

impl ExprList {
    /// Executing all the expressions, and return the last one.
    /// If no expression provided, return I64(0).
    pub fn eval(&self) -> Number {
        self.0.as_ref().map_or(Number::default(), |list| {
            let mut n = Number::default();
            for expr_rc in list.iter() {
                n = expr_rc.as_ref().eval();
            }
            n
        })
    }
}

impl Debug for ExprList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(")?;
        for member in self.0.iter() {
            write!(f, "{:?}", member)?;
        }
        write!(f, ")")
    }
}

pub struct IfCondition {
    pub cond: Box<Expr>,
    pub if_branch: ExprList,
    pub else_branch: Option<ExprList>,
}

pub enum ControlFlow {
    Condition(IfCondition),
}

impl Expr {
    pub fn eval(&self) -> Number {
        match *self {
            Expr::Number(n) => n,
            Expr::OneOp(op, ref node) => match op {
                Opcode::Sub => -node.eval(),
                _ => {
                    unreachable!();
                }
            },
            Expr::TwoOp(op, ref lnode, ref rnode) => match op {
                Opcode::Mul => lnode.eval() * rnode.eval(),
                Opcode::Div => lnode.eval() / rnode.eval(),
                Opcode::Add => lnode.eval() + rnode.eval(),
                Opcode::Sub => lnode.eval() - rnode.eval(),
                Opcode::Equal => Number::from_bool(lnode.eval() == rnode.eval()),
                Opcode::LargerOrEqual => Number::from_bool(lnode.eval() >= rnode.eval()),
                Opcode::LargerThan => Number::from_bool(lnode.eval() > rnode.eval()),
                Opcode::LessOrEqual => Number::from_bool(lnode.eval() <= rnode.eval()),
                Opcode::LessThan => Number::from_bool(lnode.eval() < rnode.eval()),

                _ => {
                    unreachable!()
                }
            },
            Expr::VarRef(ref name) => {
                let table = SYMBOL_TABLE.lock().unwrap();
                match table.get(name) {
                    Some(symbol) => symbol.value,
                    None => {
                        unimplemented!();
                    }
                }
            }
            Expr::Assign(ref name, ref rnode) => {
                let mut table = SYMBOL_TABLE.lock().unwrap();
                let v = rnode.eval();
                table.entry(name.into()).or_insert(Symbol {
                    name: name.into(),
                    value: v,
                });
                v
            }
            Expr::Flow(ref flow) => {
                match flow {
                    ControlFlow::Condition(ref flow) => {
                        // It must be a boolean value.
                        let cond = flow.cond.eval().as_bool();
                        if cond {
                            flow.if_branch.eval()
                        } else {
                            flow.else_branch
                                .as_ref()
                                .map_or(Number::default(), |branch| branch.eval())
                        }
                    }
                }
            }
        }
    }
}

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use self::Expr::*;

        match *self {
            Number(n) => write!(f, "{:?}", n),
            OneOp(op, ref node) => write!(f, "({:?}: {:?})", op, node),
            TwoOp(op, ref lnode, ref rnode) => write!(f, "({:?}: <{:?}, {:?}>)", op, lnode, rnode),
            VarRef(ref v) => write!(f, "var({:?})", v),
            Assign(ref name, ref rnode) => write!(f, "({:?} = {:?})", name, rnode),
            Flow(_) => {
                unimplemented!()
            }
        }
    }
}

// unit_test in lalrpop config will require Eq for testing.
#[cfg(test)]
impl PartialEq for Expr {
    fn eq(&self, exp: &Expr) -> bool {
        match (self, exp) {
            (Expr::Number(n1), Expr::Number(n2)) => n1 == n2,
            (Expr::OneOp(opc1, node1), Expr::OneOp(opc2, node2)) => {
                if opc1 != opc2 {
                    false
                } else {
                    node1.eq(node2)
                }
            }
            (Expr::TwoOp(opc1, lnode1, rnode1), Expr::TwoOp(opc2, lnode2, rnode2)) => {
                if opc1 != opc2 {
                    false
                } else {
                    lnode1.eq(lnode2) && rnode1.eq(rnode2)
                }
            }
            _ => false,
        }
    }
}

// Below are fields for symbol
#[derive(Clone, Debug)]
struct Symbol {
    name: String,
    value: Number,
}

lazy_static! {
    // Note: maybe using dashmap is better.
    static ref SYMBOL_TABLE: Arc<Mutex<HashMap<String, Symbol>>> = {
        Arc::new(Mutex::new(HashMap::new()))
    };
}
