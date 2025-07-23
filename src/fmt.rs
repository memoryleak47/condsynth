use std::fmt::{Debug, Formatter, Result};
use crate::*;

impl Debug for P {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            P::Var(s) => write!(f, "{s}"),
            P::IfLt(box [l, r, yes, no]) => write!(f, "(if {l:?}<{r:?} then {yes:?} else {no:?})"),
        }
    }
}


impl Debug for CounterExample {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[")?;
        for (i, (x, n)) in self.sigma.iter().enumerate() {
            write!(f, "{x}: {n}")?;
            if i != self.sigma.len()-1 {
                write!(f, ", ")?;
            }
            
        }
        write!(f, "] -> {}", self.r)
    }
}
