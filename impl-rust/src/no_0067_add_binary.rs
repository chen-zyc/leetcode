struct Solution;
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut iter_a = a.chars().rev();
        let mut iter_b = b.chars().rev();
        let mut res = String::new();
        let mut carry = 0; // 0, 1, 2, 3.
        let zero = '0' as u8;

        for _i in 0..a.len().max(b.len()) {
            iter_a.next().map(|c| {
                carry += c as u8 - zero;
            });
            iter_b.next().map(|c| {
                carry += c as u8 - zero;
            });
            res.insert(0, ((carry & 1) + zero) as char); // 取最后一位
            carry = carry / 2; // 取进位
        }
        if carry > 0 {
            res.insert(0, (carry + zero) as char);
        }

        res
    }

    // 链接：https://leetcode-cn.com/problems/add-binary/solution/rust-by-ruislan-5/
    pub fn add_binary2(mut a: String, mut b: String) -> String {
        let mut carry = 0;
        let mut res = String::new();
        loop {
            let (n1, n2) = (a.pop(), b.pop());

            if n1 == None && n2 == None {
                break;
            }
            let mut sum = carry;
            if let Some(x) = n1 {
                sum += x.to_digit(2).unwrap();
            }
            if let Some(x) = n2 {
                sum += x.to_digit(2).unwrap();
            }
            carry = sum / 2;
            res.insert_str(0, &(sum % 2).to_string());
        }
        if carry > 0 {
            res.insert_str(0, &carry.to_string());
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_binary_example1() {
        let res = Solution::add_binary("11".to_owned(), "1".to_owned());
        assert_eq!(res, "100".to_owned());
    }

    #[test]
    fn test_add_binary_example2() {
        let res = Solution::add_binary("1010".to_owned(), "1011".to_owned());
        assert_eq!(res, "10101".to_owned());
    }
}
