use crate::*;

pub type Nat = u32;

pub enum P {
    Var(Symbol),
    IfLt(Box<[P; 4]>),
}

pub type Sigma = Map<Symbol, Nat>;

pub struct CounterExample {
    sigma: Sigma,
    r: Nat,
}

pub type Problem = fn(&P) -> Option<CounterExample>;
pub type Synthesizer = fn(&[CounterExample]) -> P;
