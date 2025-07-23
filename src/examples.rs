use crate::*;

pub fn max3(p: &P) -> Option<CounterExample> {
    let vs = &[
        (22, 4, 2),
        (22, 5, 4),
        (22, 400, 2),

        (0, 1, 2),
        (0, 2, 1),
        (1, 0, 2),
        (1, 2, 0),
        (2, 0, 1),
        (2, 1, 0),
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

pub fn max6(p: &P) -> Option<CounterExample> {
    let vs = &[
        [0, 1, 2, 4, 2, 3],
        [22, 4, 2, 2, 4, 2],
        [22, 5, 4, 8, 7, 6],
        [22, 5, 400, 8, 7, 6],
        [22, 5, 40, 800, 7, 6],
        [22, 5, 40, 4, 700, 6],
        [22, 5, 40, 4, 70, 99],
        [22, 200, 40, 4, 70, 99],
        // TODO add more entries (automatically) to make this complete.
    ];

    for &v in vs {
        let r = v.iter().copied().fold(0, |x, y| x.max(y));
        let mut sigma = Sigma::new();
        for i in 0..6 {
            let symb = if i == 0 { String::new() } else { i.to_string() };
            let symb = Symbol::new(format!("x{symb}"));
            sigma.insert(symb, v[i]);
        }
        if eval(p, &sigma) != r {
            let ce = CounterExample { sigma, r };
            return Some(ce);
        }
    }
    None
}
