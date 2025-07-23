use std::fmt::{Debug, Formatter, Result};
use crate::*;

impl Debug for P {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            P::Var(i) => write!(f, "x{i}"),
            P::IfLt(box [l, r, yes, no]) => write!(f, "(if {l:?}<{r:?} then {yes:?} else {no:?})"),
        }
    }
}


impl Debug for CounterExample {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[")?;
        for (i, n) in self.sigma.iter().enumerate() {
            write!(f, "x{i}: {n}")?;
            if i != self.sigma.len()-1 {
                write!(f, ", ")?;
            }
            
        }
        write!(f, "] -> {}", self.r)
    }
}

pub fn size(p: &P) -> usize {
    match p {
        P::Var(_) => 1,
        P::IfLt(l) => 1 + l.iter().map(size).sum::<usize>(),
    }
}
