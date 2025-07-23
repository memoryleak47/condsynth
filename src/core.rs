use crate::*;

pub type Nat = u32;

pub enum P {
    Var(Symbol),
    IfLt(Box<[P; 4]>),
}

pub type Sigma = Map<Symbol, Nat>;

#[derive(Clone)]
pub struct CounterExample {
    pub sigma: Sigma,
    pub r: Nat,
}

pub type Problem = fn(&P) -> Option<CounterExample>;
pub type Synthesizer = fn(&[CounterExample]) -> P;

pub fn eval(p: &P, sigma: &Sigma) -> Nat {
    match p {
        P::Var(s) => sigma[s],
        P::IfLt(box [l, r, yes, no]) => {
            if eval(l, sigma) < eval(r, sigma) {
                eval(yes, sigma)
            } else {
                eval(no, sigma)
            }
        },
    }
}

pub fn cegis(problem: Problem, synth: Synthesizer) -> P {
    let mut ces = Vec::new();
    loop {
        let p = synth(&ces);
        dbg!(&ces);
        dbg!(&p);
        assert!(ces.iter().all(|ce| eval(&p, &ce.sigma) == ce.r));

        if let Some(p2) = problem(&p) {
            ces.push(p2);
        } else {
            return p;
        }
    }
}
