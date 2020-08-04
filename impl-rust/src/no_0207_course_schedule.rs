struct Solution;

#[derive(Debug, Clone, PartialEq)]
enum State {
    Unstarted,
    Doing,
    Finished,
}

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        // 广度优先解法
        let mut edges = vec![Vec::new(); num_courses as usize];
        let mut indeg = vec![0; num_courses as usize]; // 入度
        for info in prerequisites {
            // info[1] -> info[0]
            edges[info[1] as usize].push(info[0] as usize);
            indeg[info[0] as usize] += 1;
        }

        // 或者 let mut q = (0..num_courses as usize).filter(|&i| indeg[i] == 0).collect::<Vec<_>>();
        let mut q = Vec::new();
        for i in 0..num_courses as usize {
            if indeg[i] == 0 {
                q.push(i);
            }
        }

        let mut result_count = 0;
        while let Some(u) = q.pop() {
            result_count += 1;
            for v in edges[u].iter() {
                indeg[*v] -= 1;
                if indeg[*v] == 0 {
                    q.push(*v);
                }
            }
        }

        result_count == num_courses
    }

    pub fn can_finish1(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        // 深度优先解法
        let mut edges = vec![Vec::new(); num_courses as usize];
        for info in prerequisites {
            // info[1] -> info[0]
            edges[info[1] as usize].push(info[0] as usize);
        }

        let mut states = vec![State::Unstarted; num_courses as usize];

        for i in 0..num_courses as usize {
            if states[i] == State::Unstarted {
                if !Self::dfs(i, &mut states, &edges) {
                    return false;
                }
            }
        }

        true
    }

    fn dfs(u: usize, states: &mut Vec<State>, edges: &Vec<Vec<usize>>) -> bool {
        states[u] = State::Doing;
        for v in edges[u].iter() {
            match states[*v] {
                State::Unstarted => {
                    if !Self::dfs(*v, states, edges) {
                        // 有环
                        return false;
                    }
                }
                State::Doing => {
                    // 有环
                    return false;
                }
                State::Finished => {}
            }
        }
        states[u] = State::Finished;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_finish1() {
        let ok = Solution::can_finish(2, vec![vec![1, 0]]);
        assert!(ok);
    }

    #[test]
    fn test_can_finish2() {
        let ok = Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]);
        assert!(!ok);
    }
}
