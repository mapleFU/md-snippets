pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Opcode, Box<Expr>),
}

pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

// Note: we need to represent not only integers.
#[derive(Clone, Copy, Debug)]
pub enum Number {
    I64(i64),
    F64(f64),
}

impl Default for Number {
    fn default() -> Self { 
        Number::I64(0)
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
            Number::I64(i) => {
                i as f64
            },
            Number::F64(f) => {
                f
            }
        }
    }
}

impl std::ops::Add for Number {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Number::I64(i1), Number::I64(i2)) => {
                Number::I64(i1 + i2)
            },
            (v1, v2) => {
                Number::F64(v1.as_f64() + v2.as_f64())
            }
        }
    }
}

impl std::ops::Mul for Number {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Number::I64(i1), Number::I64(i2)) => {
                Number::I64(i1 * i2)
            },
            (v1, v2) => {
                Number::F64(v1.as_f64() * v2.as_f64())
            }
        }
    }
}

impl std::ops::Div for Number {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            (Number::I64(i1), Number::I64(i2)) => {
                Number::I64(i1 * i2)
            },
            (v1, v2) => {
                Number::F64(v1.as_f64() * v2.as_f64())
            }
        }
    }
}