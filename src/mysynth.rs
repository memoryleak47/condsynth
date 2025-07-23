use crate::*;

fn delta(x: usize, y: usize) -> usize {
    if x > y { x-y } else { y-x }
}

pub fn mysynth(ces: &[CounterExample]) -> P {
    if ces.is_empty() {
        return P::Var(Symbol::new("x"));
    }

    let vars: Vec<Symbol> = ces[0].sigma.keys().cloned().collect();

    for x in &vars {
        if ces.iter().all(|ce| ce.r == ce.sigma[x]) {
            return P::Var(*x);
        }
    }

    let mut best_x = Symbol::new("x");
    let mut best_y = Symbol::new("y");
    let mut best_cost = usize::MAX;

    for x in &vars {
        for y in &vars {
            let pair = (x, y);
            let m = ces.iter().filter(|ce| ce.sigma[x] < ce.sigma[y]).count();
            let n = ces.len();
            let cost = delta(2*m, n);
            if cost < best_cost {
                best_x = *x;
                best_y = *y;
                best_cost = cost;
            }
        }
    }

    let (x, y) = (best_x, best_y);
    let l: Vec<_> = ces.iter().filter(|ce| ce.sigma[&x] < ce.sigma[&y]).cloned().collect();
    let r: Vec<_> = ces.iter().filter(|ce| ce.sigma[&x] >= ce.sigma[&y]).cloned().collect();
    
    let l = mysynth(&l);
    let r = mysynth(&r);

    P::IfLt(Box::new([P::Var(x), P::Var(y), l, r]))
}
