struct Solution;
impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let iter1 = BackspaceIter::new(s.chars().rev());
        let iter2 = BackspaceIter::new(t.chars().rev());
        iter1.eq(iter2)
    }
}

struct BackspaceIter<I> {
    iter: I,
}

impl<I> BackspaceIter<I> {
    fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<I> Iterator for BackspaceIter<I>
where
    I: Iterator<Item = char>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let mut num_skip = 0;
        while let Some(c) = self.iter.next() {
            if c == '#' {
                num_skip += 1;
                continue;
            }
            if num_skip > 0 {
                num_skip -= 1;
                continue;
            }
            return Some(c);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backspace_compare() {
        assert_eq!(
            Solution::backspace_compare("ab#c".to_owned(), "ad#c".to_owned()),
            true
        );
        assert_eq!(
            Solution::backspace_compare("ab##".to_owned(), "c#d#".to_owned()),
            true
        );
        assert_eq!(
            Solution::backspace_compare("a##c".to_owned(), "#a#c".to_owned()),
            true
        );
        assert_eq!(
            Solution::backspace_compare("a#c".to_owned(), "b".to_owned()),
            false
        );
    }
}
