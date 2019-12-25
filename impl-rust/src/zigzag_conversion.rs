pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows <= 1 {
        return s;
    }
    let s = s.as_bytes();
    let num_rows = num_rows as usize;
    let n = s.len();
    let mut ret = String::with_capacity(n);
    let cycle_len = num_rows + num_rows - 2;
    for i in 0..num_rows {
        for j in (0..).step_by(cycle_len as usize).take_while(|x| *x + i < n) {
            ret.push(s[i + j] as char);
            // 如果不是第一行和最后一行，那就有两个字符
            if i != 0 && i != num_rows - 1 && j + cycle_len - i < n {
                ret.push(s[j + cycle_len - i] as char);
            }
        }
    }
    ret
}

#[cfg(test)]
mod test {
    use crate::zigzag_conversion::convert;

    #[test]
    fn test_convert() {
        let s = "LEETCODEISHIRING";
        let num_rows = 3;
        assert_eq!(
            convert(s.to_owned(), num_rows),
            "LCIRETOESIIGEDHN".to_owned()
        );

        let s = "LEETCODEISHIRING";
        let num_rows = 4;
        assert_eq!(
            convert(s.to_owned(), num_rows),
            "LDREOEIIECIHNTSG".to_owned()
        );
    }
}
