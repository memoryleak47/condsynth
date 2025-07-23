use crate::*;

fn delta(x: usize, y: usize) -> usize {
    if x > y { x-y } else { y-x }
}

fn heur_cost(l: &[&CounterExample], r: &[&CounterExample], num_vars: usize) -> usize {
    // old heuristic: delta(l.len(), r.len())
    varcnt(l, num_vars).max(varcnt(r, num_vars))
}

fn varcnt(ces: &[&CounterExample], num_vars: usize) -> usize {
    if ces.is_empty() { return 0; }

    let mut best_score = 0;
    let mut best_var = 0;
    for v in 0..num_vars {
        let score = ces.iter().filter(|ce| ce.sigma[v] == ce.r).count();
        if score > best_score {
            best_var = v;
            best_score = score;
        }
    }
    let rest: Vec<&CounterExample> = ces.iter().copied().filter(|ce| ce.sigma[best_var] != ce.r).collect();
    1 + varcnt(&*rest, num_vars)
}


pub struct MySynth;
impl Synth for MySynth {
    fn synth<'a>(&self, ces: impl Iterator<Item=&'a CounterExample> + Clone, num_vars: usize) -> P {
        if ces.clone().all(|_| false) {
            return P::Var(0);
        }

        for x in 0..num_vars {
            if ces.clone().all(|ce| ce.r == ce.sigma[x]) {
                return P::Var(x);
            }
        }

        let mut best_x = 0;
        let mut best_y = 0;
        let mut best_cost = usize::MAX;

        let mut best_l: Vec<&CounterExample> = Vec::new();
        let mut best_r: Vec<&CounterExample> = Vec::new();

        for x in 0..num_vars {
            for y in 0..num_vars {
                let mut l: Vec<&CounterExample> = Vec::new();
                let mut r: Vec<&CounterExample> = Vec::new();
                for ce in ces.clone() {
                    if ce.sigma[x] < ce.sigma[y] {
                        l.push(ce);
                    } else {
                        r.push(ce);
                    }
                }
                let cost = heur_cost(&*l, &*r, num_vars);
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

        let l = self.synth(best_l.into_iter(), num_vars);
        let r = self.synth(best_r.into_iter(), num_vars);

        P::IfLt(Box::new([P::Var(best_x), P::Var(best_y), l, r]))
    }
}
