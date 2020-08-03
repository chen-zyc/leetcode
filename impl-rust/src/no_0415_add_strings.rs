struct Solution;
impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let (mut iter1, mut iter2) = (num1.chars().rev(), num2.chars().rev());
        let mut carry = 0;
        let mut ans = Vec::new();
        let base = '0' as u8;

        loop {
            match (iter1.next(), iter2.next()) {
                (Some(c1), Some(c2)) => {
                    let sum = c1 as u8 - base + c2 as u8 - base + carry;
                    carry = sum / 10;
                    ans.push((base + sum % 10) as char);
                }
                (Some(c), None) => {
                    let sum = c as u8 - base + carry;
                    carry = sum / 10;
                    ans.push((base + sum % 10) as char);
                }
                (None, Some(c)) => {
                    let sum = c as u8 - base + carry;
                    carry = sum / 10;
                    ans.push((base + sum % 10) as char);
                }
                (None, None) => {
                    if carry > 0 {
                        ans.push((base + carry) as char);
                    }
                    break;
                }
            }
        }

        ans.into_iter().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_strings() {
        assert_eq!(
            Solution::add_strings("123".to_string(), "34".to_string()),
            "157".to_string()
        );
    }
}
