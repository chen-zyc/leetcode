
pub fn climb_stairs(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }

    let mut first = 1;
    let mut second = 2;
    for _ in 3..=n {
        let sum = first+second;
        first = second;
        second = sum;
    }
    second
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_climb_stairs() {
        assert_eq!(1, climb_stairs(1));
        assert_eq!(2, climb_stairs(2));
        assert_eq!(3, climb_stairs(3));
        assert_eq!(5, climb_stairs(4));
    }
}