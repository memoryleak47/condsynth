use crate::*;

pub fn enumerated_problem<F: Fn(&Sigma) -> Nat>(sig: Vec<Symbol>, f: F) -> impl Problem {
    struct EnumeratedProblem<F: Fn(&Sigma) -> Nat> {
        sig: Vec<Symbol>,
        sigmas: Vec<Sigma>,
        f: F,
    }

    impl<F: Fn(&Sigma) -> Nat> Problem for EnumeratedProblem<F> {
        fn signature(&self) -> &[Symbol] { &*self.sig }
        fn check(&self, p: &P) -> Option<CounterExample> {
            for sigma in &self.sigmas {
                let r = (self.f)(sigma);
                if eval(p, &sigma) != r {
                    let ce = CounterExample { sigma: sigma.clone(), r };
                    return Some(ce);
                }
            }
            None
        }
    }

    EnumeratedProblem {
        sigmas: sigmas(&sig, sig.len() as _),
        sig: sig,
        f,
    }
}

fn sigmas(sig: &[Symbol], n: u32) -> Vec<Sigma> {
    if sig.is_empty() {
        return vec![Sigma::new()];
    }

    let mut outs = Vec::new();
    for rest in sigmas(&sig[1..], n) {
        for x in 0..n {
            let mut sigma = Sigma::new();
            sigma.insert(sig[0], x);
            sigma.extend(&rest);
            outs.push(sigma);
        }
    }
    outs
}

pub fn max3() -> impl Problem {
    let x = Symbol::new("x");
    let y = Symbol::new("y");
    let z = Symbol::new("z");
    let sig = vec![x, y, z];
    enumerated_problem(sig, move |sigma| {
        sigma[&x].max(sigma[&y]).max(sigma[&z])
    })
}
