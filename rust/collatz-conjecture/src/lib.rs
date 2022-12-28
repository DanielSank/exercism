fn next(n: u64) -> Option<u64> {
    match n % 2 {
        0 => Some(n / 2),
        1 => {
            if n >= u64::MAX / 3 { return None }
            let candidate = 3 * n + 1;
            return Some(candidate);
        },
        _ => panic!() // unreachable
    }
}

pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 { return None }
    if n == 1 {return Some(0) }
    let mut candidate: u64 = n;
    let mut steps: u64 = 0;
    loop {
        steps = steps + 1;
        match next(candidate) {
            Some(1) => { return Some(steps) },
            Some(x) => { candidate = x; }
            None => return None,
        }
    }
}
