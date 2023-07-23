fn exponentiate(num: u32, exp: u32, p: u32) -> u32 {
    num.pow(exp) % p
}

fn solve(x: u32, k: u32, c: u32, q: u32) -> u32 {
    // s = (k - c * x) mod q
    let s = (k as i32 - (c * x) as i32) % q as i32;
    if s >= 0 {
        s as u32
    } else {
        assert!(s + q as i32 >= 0);
        (s + q as i32) as u32
    }
}

fn main() {
    println!("{}", 4 % 3);
    println!("{}", -4 % 3);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_exponentiate() {
        assert_eq!(exponentiate(1, 2, 11), 1);
        assert_eq!(exponentiate(2, 3, 11), 8);
        assert_eq!(exponentiate(2, 4, 11), 5);
    }

    #[test]
    fn test_solve() {
        // (10 - 2 * 1) mod 101 = 8
        assert_eq!(solve(2, 10, 1, 101), 8);
        // (10 - 2 * 6) mod 101 = 99
        assert_eq!(solve(2, 10, 6, 101), 99);
    }
}
