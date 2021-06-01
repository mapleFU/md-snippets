use std::fmt::Debug;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Note: we need to represent not only integers.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Number {
    I64(i64),
    F64(f64),
}

impl Default for Number {
    fn default() -> Self {
        Number::I64(0)
    }
}

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl std::fmt::Display for Number {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        match self {
            Number::F64(fv) => {
                write!(f, "F64({})", fv)
            }
            Number::I64(iv) => {
                write!(f, "I64({})", iv)
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

    pub fn as_f64(self) -> f64 {
        match self {
            Number::I64(i) => i as f64,
            Number::F64(f) => f,
        }
    }

    pub fn as_i64(self) -> i64 {
        match self {
            Number::I64(i) => i,
            Number::F64(f) => f as i64,
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
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum BuiltinFunc {
    Sqrt,
    Exp,
    Log,
    Print,
}

pub enum Expr {
    Number(Number),
    OneOp(Opcode, Box<Expr>),
    TwoOp(Opcode, Box<Expr>, Box<Expr>),
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
                _ => {
                    unreachable!()
                },
            },
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
struct Symbol {
    name: String,
    value: Number,
}

lazy_static! {
    // Note: maybe using dashmap is better.
    static ref SymbolTable: Arc<Mutex<HashMap<String, Symbol>>> = {
        Arc::new(Mutex::new(HashMap::new()))
    };
}