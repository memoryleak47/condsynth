use crate::*;

pub fn max3(p: &P) -> Option<CounterExample> {
    let vs = &[
        (0, 1, 2),
        (22, 4, 2),
        (22, 5, 4),
    ];

    let s_x = Symbol::from("x");
    let s_y = Symbol::from("y");
    let s_z = Symbol::from("z");

    for &(x, y, z) in vs {
        let r = x.max(y).max(z);
        let mut sigma = Sigma::new();
        sigma.insert(s_x, x);
        sigma.insert(s_y, y);
        sigma.insert(s_z, z);
        if eval(p, &sigma) != r {
            let ce = CounterExample { sigma, r };
            return Some(ce);
        }
    }
    None
}
