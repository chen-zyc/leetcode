struct Solution;
impl Solution {
    pub fn pattern_matching(mut pattern: String, value: String) -> bool {
        // a 和 b 出现的次数
        let (mut count_a, mut count_b) = pattern.chars().fold((0, 0), |acc, c| {
            if c == 'a' {
                (acc.0 + 1, acc.1)
            } else {
                (acc.0, acc.1 + 1)
            }
        });
        if count_a < count_b {
            // 把 a 和 b 互换
            pattern = pattern
                .chars()
                .map(|c| if c == 'a' { 'b' } else { 'a' })
                .collect();
            std::mem::swap(&mut count_a, &mut count_b);
        }
        println!("count_a = {}, count_b = {}", count_a, count_b);
        if value.len() == 0 {
            return count_b == 0;
        }
        if pattern.len() == 0 {
            return false; // value 不为空。
        }

        // 此时 pattern 和 value 都不为空。
        let pattern = pattern.as_bytes();
        let value = value.as_bytes();
        for len_a in (0..=value.len()).take_while(|x| *x * count_a <= value.len()) {
            // B 占的所有字符
            let rest = value.len() - len_a * count_a;
            println!("len_a = {}, rest = {}", len_a, rest);
            // 保证一个 B 占用的字符数是整数
            if (count_b == 0 && rest == 0) || (count_b > 0 && rest % count_b == 0) {
                let len_b = if count_b == 0 { 0 } else { rest / count_b };
                println!("len_a = {}, len_b = {}", len_a, len_b);

                let mut pos = 0;
                let mut correct = true;
                let mut value_a = None;
                let mut value_b = None;
                for p in pattern.iter() {
                    if *p == 'a' as u8 {
                        let sub = &value[pos..pos + len_a];
                        println!("a sub = {:?}", String::from_utf8_lossy(sub));
                        if value_a.is_none() {
                            value_a = Some(sub);
                        } else if value_a != Some(sub) {
                            correct = false;
                            break;
                        }
                        pos += len_a;
                    } else {
                        let sub = &value[pos..pos + len_b];
                        println!("b sub = {:?}", String::from_utf8_lossy(sub));
                        if value_b.is_none() {
                            value_b = Some(sub);
                        } else if value_b != Some(sub) {
                            correct = false;
                            break;
                        }
                        pos += len_b;
                    }
                }
                if correct && value_a != value_b {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_matching_example1() {
        assert_eq!(
            Solution::pattern_matching("abba".to_owned(), "dogcatcatdog".to_owned()),
            true
        );
    }

    #[test]
    fn test_pattern_matching_example2() {
        assert_eq!(
            Solution::pattern_matching("abba".to_owned(), "dogcatcatfish".to_owned()),
            false
        );
    }

    #[test]
    fn test_pattern_matching_example3() {
        assert_eq!(
            Solution::pattern_matching("aaaa".to_owned(), "dogcatcatdog".to_owned()),
            false
        );
    }

    #[test]
    fn test_pattern_matching_example4() {
        assert_eq!(
            Solution::pattern_matching("abba".to_owned(), "dogdogdogdog".to_owned()),
            true
        );
    }

    #[test]
    fn test_pattern_matching_example5() {
        assert_eq!(
            Solution::pattern_matching("a".to_owned(), "zqvamqvuuvvazv".to_owned()),
            true
        );
    }
}
