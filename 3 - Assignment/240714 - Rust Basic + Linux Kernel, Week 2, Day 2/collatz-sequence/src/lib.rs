/// Determine the length of the collatz sequence beginning at `n`.
pub fn collatz_length(mut n: i32) -> u32 {
    let mut i = 1u32;
    loop {
        if n == 1 {
            break i;
        } else if n % 2 == 1 {
            n = 3*n + 1;
        } else {
            n = n / 2;
        }
        i = i + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collatz_1() {
        let ret = collatz_length(1);
        assert_eq!(ret, 1);
    }

    #[test]
    fn collatz_2() {
        let ret = collatz_length(2);
        assert_eq!(ret, 2);
    }

    #[test]
    fn collatz_5() {
        let ret = collatz_length(5);
        assert_eq!(ret, 6);
    }

    #[test]
    fn collatz_11() {
        let ret = collatz_length(11);
        assert_eq!(ret, 15);
    }

    #[test]
    fn collatz_27() {
        let ret = collatz_length(27);
        assert_eq!(ret, 112);
    }
}
