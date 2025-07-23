use crate::*;

fn delta(x: usize, y: usize) -> usize {
    if x > y { x-y } else { y-x }
}

fn heur_cost(l: &[CounterExample], r: &[CounterExample], num_vars: usize) -> usize {
    // old heuristic: delta(l.len(), r.len())
    varcnt(l, num_vars).max(varcnt(r, num_vars))
}

fn varcnt(ces: &[CounterExample], num_vars: usize) -> usize {
    if ces.is_empty() { return 0; }

    let mut ces: Vec<CounterExample> = ces.iter().cloned().collect();

    let mut best_score = 0;
    let mut best_var = 0;
    for v in 0..num_vars {
        let score = ces.iter().filter(|ce| ce.sigma[v] == ce.r).count();
        if score > best_score {
            best_var = v;
            best_score = score;
        }
    }
    let rest: Vec<_> = ces.iter().filter(|ce| ce.sigma[best_var] != ce.r).cloned().collect();
    1 + varcnt(&*rest, num_vars)
}

pub fn mysynth(ces: &[CounterExample], num_vars: usize) -> P {
    if ces.is_empty() {
        return P::Var(0);
    }

    for x in 0..num_vars {
        if ces.iter().all(|ce| ce.r == ce.sigma[x]) {
            return P::Var(x);
        }
    }

    let mut best_x = 0;
    let mut best_y = 0;
    let mut best_cost = usize::MAX;

    let mut best_l = Vec::new();
    let mut best_r = Vec::new();

    for x in 0..num_vars {
        for y in 0..num_vars {
            let mut l = Vec::new();
            let mut r = Vec::new();
            for ce in ces.iter() {
                if ce.sigma[x] < ce.sigma[y] {
                    l.push(ce.clone());
                } else {
                    r.push(ce.clone());
                }
            }
            let cost = heur_cost(&l, &r, num_vars);
            if cost < best_cost {
                best_x = x;
                best_y = y;
                best_cost = cost;
                best_l = l;
                best_r = r;
            }
        }
    }

    assert!(best_l.len() > 0);
    assert!(best_r.len() > 0);

    let l = mysynth(&best_l, num_vars);
    let r = mysynth(&best_r, num_vars);

    P::IfLt(Box::new([P::Var(best_x), P::Var(best_y), l, r]))
}
