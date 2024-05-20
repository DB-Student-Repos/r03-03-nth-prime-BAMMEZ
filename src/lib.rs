fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

pub fn nth(n: usize) -> u32 {
    let mut count = 0;
    let mut candidate = 2;

    loop {
        if is_prime(candidate) {
            count += 1;
            if count == n + 1 {
                return candidate;
            }
        }
        candidate += 1;
    }
}


