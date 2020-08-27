struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        // 转成 map，出发点是 key，终点列表是 value，并且 value 是排序了的。
        let mut map = HashMap::new();
        for mut ticket in tickets.into_iter() {
            let values = map.entry(ticket.remove(0)).or_insert(Vec::new());
            values.push(ticket.remove(0));
        }
        for (_, values) in map.iter_mut() {
            values.sort();
        }

        let mut ans = Vec::new();
        Self::find_itinerary_dfs("JFK".to_owned(), &mut map, &mut ans);
        ans.reverse();
        ans
    }

    fn find_itinerary_dfs(
        begin: String,
        tickets: &mut HashMap<String, Vec<String>>,
        ans: &mut Vec<String>,
    ) {
        loop {
            match tickets.get_mut(&begin) {
                None => {
                    break;
                }
                Some(values) => {
                    if values.len() == 0 {
                        break;
                    }

                    Self::find_itinerary_dfs(values.remove(0), tickets, ans);
                }
            }
        }
        ans.push(begin);
    }

    pub fn find_itinerary_failed(tickets: Vec<Vec<String>>) -> Vec<String> {
        // 转成 map，出发点是 key，终点列表是 value，并且 value 是排序了的。
        let mut map = HashMap::new();
        for mut ticket in tickets.into_iter() {
            let values = map.entry(ticket.remove(0)).or_insert(Vec::new());
            values.push(ticket.remove(0));
        }
        for (_, values) in map.iter_mut() {
            values.sort_by(|x, y| y.cmp(x));
        }

        // 从 JFK 出发
        let mut ans = vec!["JFK".to_owned()];
        let mut begin = ans.last().unwrap();
        loop {
            // 获取到终点列表。
            match map.get_mut(begin) {
                // ERROR!! 如果获取到最后一个后不能有效的走完全部的行程，那么需要尝试其它路径。
                // 获取到列表的最后一个。
                Some(values) => match values.pop() {
                    Some(end) => {
                        ans.push(end);
                        begin = ans.last().unwrap();
                    }
                    None => break,
                },
                None => break,
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_itinerary1() {
        let tickers = vec![
            vec!["MUC", "LHR"],
            vec!["JFK", "MUC"],
            vec!["SFO", "SJC"],
            vec!["LHR", "SFO"],
        ]
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|s| s.to_owned())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();
        let ans = Solution::find_itinerary(tickers);
        let want = vec!["JFK", "MUC", "LHR", "SFO", "SJC"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        assert_eq!(ans, want);
    }

    #[test]
    fn test_find_itinerary2() {
        let tickers = vec![
            vec!["JFK", "SFO"],
            vec!["JFK", "ATL"],
            vec!["SFO", "ATL"],
            vec!["ATL", "JFK"],
            vec!["ATL", "SFO"],
        ]
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|s| s.to_owned())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();
        let ans = Solution::find_itinerary(tickers);
        let want = vec!["JFK", "ATL", "JFK", "SFO", "ATL", "SFO"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        assert_eq!(ans, want);
    }

    #[test]
    fn test_find_itinerary3() {
        let tickers = vec![vec!["JFK", "KUL"], vec!["JFK", "NRT"], vec!["NRT", "JFK"]]
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|s| s.to_owned())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>();
        let ans = Solution::find_itinerary(tickers);
        let want = vec!["JFK", "NRT", "JFK", "KUL"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        assert_eq!(ans, want);
    }
}
