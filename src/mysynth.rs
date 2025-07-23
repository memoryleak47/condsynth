use crate::*;

fn delta(x: usize, y: usize) -> usize {
    if x > y { x-y } else { y-x }
}

fn heur_cost(l: &[CounterExample], r: &[CounterExample]) -> usize {
    delta(l.len(), r.len())
}

pub fn mysynth(ces: &[CounterExample], vars: &[Symbol]) -> P {
    if ces.is_empty() {
        return P::Var(vars[0]);
    }

    for x in vars {
        if ces.iter().all(|ce| ce.r == ce.sigma[x]) {
            return P::Var(*x);
        }
    }

    let mut best_x = vars[0];
    let mut best_y = vars[0];
    let mut best_cost = usize::MAX;

    let mut best_l = Vec::new();
    let mut best_r = Vec::new();

    for x in vars {
        for y in vars {
            let mut l = Vec::new();
            let mut r = Vec::new();
            for ce in ces.iter() {
                if ce.sigma[x] < ce.sigma[y] {
                    l.push(ce.clone());
                } else {
                    r.push(ce.clone());
                }
            }
            let cost = heur_cost(&l, &r);
            if cost < best_cost {
                best_x = *x;
                best_y = *y;
                best_cost = cost;
                best_l = l;
                best_r = r;
            }
        }
    }

    let l = mysynth(&best_l, vars);
    let r = mysynth(&best_r, vars);

    P::IfLt(Box::new([P::Var(best_x), P::Var(best_y), l, r]))
}
