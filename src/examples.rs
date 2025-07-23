use crate::*;

pub fn enumerated_problem<F: Fn(&Sigma) -> Nat>(num_vars: usize, f: F) -> impl Problem {
    struct EnumeratedProblem<F: Fn(&Sigma) -> Nat> {
        num_vars: usize,
        sigmas: Vec<Sigma>,
        f: F,
    }

    impl<F: Fn(&Sigma) -> Nat> Problem for EnumeratedProblem<F> {
        fn num_vars(&self) -> usize { self.num_vars }
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
        num_vars,
        sigmas: sigmas(0, num_vars),
        f,
    }
}

fn sigmas(i: usize, num_vars: usize) -> Vec<Sigma> {
    if i == num_vars {
        return vec![Sigma::new()];
    }

    let mut outs = Vec::new();
    for rest in sigmas(i+1, num_vars) {
        for x in 0..num_vars {
            let mut sigma = Sigma::new();
            sigma.push(x);
            sigma.extend(&rest);
            outs.push(sigma);
        }
    }
    outs
}

pub fn max_n(n: usize) -> impl Problem {
    assert!(n > 0);

    enumerated_problem(n, move |sigma| {
        sigma.iter().copied().fold(0, |x, y| x.max(y))
    })
}
