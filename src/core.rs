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

pub trait Problem {
    fn signature(&self) -> &[Symbol];
    fn check(&self, p: &P) -> Option<CounterExample>;
}

pub type Synthesizer = fn(&[CounterExample], &[Symbol]) -> P;

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

pub fn cegis(problem: impl Problem, synth: Synthesizer) -> P {
    let mut ces = Vec::new();
    loop {
        let p = synth(&ces, problem.signature());
        dbg!(&p);
        assert!(ces.iter().all(|ce| eval(&p, &ce.sigma) == ce.r));

        if let Some(ce) = problem.check(&p) {
            dbg!(&ce);
            ces.push(ce);
        } else {
            return p;
        }
    }
}
