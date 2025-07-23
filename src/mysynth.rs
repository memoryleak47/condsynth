use crate::*;

fn delta(x: usize, y: usize) -> usize {
    if x > y { x-y } else { y-x }
}

fn heur_cost(l: &[CounterExample], r: &[CounterExample], sig: &[Symbol]) -> usize {
    // old heuristic: delta(l.len(), r.len())
    varcnt(l, sig).max(varcnt(r, sig))
}

fn varcnt(ces: &[CounterExample], sig: &[Symbol]) -> usize {
    if ces.is_empty() { return 0; }

    let mut ces: Vec<CounterExample> = ces.iter().cloned().collect();

    let mut best_score = 0;
    let mut best_var = sig[0];
    for v in sig {
        let score = ces.iter().filter(|ce| ce.sigma[v] == ce.r).count();
        if score > best_score {
            best_var = *v;
            best_score = score;
        }
    }
    let rest: Vec<_> = ces.iter().filter(|ce| ce.sigma[&best_var] != ce.r).cloned().collect();
    1 + varcnt(&*rest, sig)
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
            let cost = heur_cost(&l, &r, vars);
            if cost < best_cost {
                best_x = *x;
                best_y = *y;
                best_cost = cost;
                best_l = l;
                best_r = r;
            }
        }
    }

    assert!(best_l.len() > 0);
    assert!(best_r.len() > 0);

    let l = mysynth(&best_l, vars);
    let r = mysynth(&best_r, vars);

    P::IfLt(Box::new([P::Var(best_x), P::Var(best_y), l, r]))
}
