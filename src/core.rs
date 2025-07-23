use crate::*;

pub type Nat = usize;

pub enum P {
    Var(Var),
    IfLt(Box<[P; 4]>),
}

pub type Sigma = Vec<Nat>;

#[derive(Clone)]
pub struct CounterExample {
    pub sigma: Sigma,
    pub r: Nat,
}

pub type Var = usize;

pub trait Problem {
    fn num_vars(&self) -> usize;
    fn check(&self, p: &P) -> Option<CounterExample>;
}

pub trait Synth {
    fn synth<'a>(&self, it: impl Iterator<Item=&'a CounterExample> + Clone, num_vars: usize) -> P;
}

pub fn eval(p: &P, sigma: &Sigma) -> Nat {
    match p {
        P::Var(s) => sigma[*s],
        P::IfLt(box [l, r, yes, no]) => {
            if eval(l, sigma) < eval(r, sigma) {
                eval(yes, sigma)
            } else {
                eval(no, sigma)
            }
        },
    }
}

pub fn cegis(problem: impl Problem, synth: impl Synth) -> P {
    let mut ces = Vec::new();
    loop {
        let p = synth.synth(ces.iter(), problem.num_vars());
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
