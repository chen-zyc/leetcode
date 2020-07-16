struct Solution;

#[derive(Debug, Clone, PartialEq)]
enum Color {
    UNCOLORED,
    RED,
    GREEN,
}

impl Solution {
    // 广度优先
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let n = graph.len();
        let mut color = vec![Color::UNCOLORED; n];
        for i in 0..n {
            // 已经染过色了，不处理它了
            if color[i] != Color::UNCOLORED {
                continue;
            }

            let mut queue = Vec::new();
            queue.push(i);
            // 当前点染成红色
            color[i] = Color::RED;

            while let Some(node) = queue.pop() {
                let next_color = if color[node] == Color::RED {
                    Color::GREEN
                } else {
                    Color::RED
                };
                for neighbor in graph[node].iter() {
                    let neighbor = *neighbor as usize;
                    match &color[neighbor] {
                        Color::UNCOLORED => {
                            // 把相邻的点染成 next_color，并且放到 queue 中。
                            color[neighbor] = next_color.clone();
                            queue.push(neighbor);
                        }
                        color if color != &next_color => {
                            // 已经被染色了，但不是我们期望的颜色
                            return false;
                        }
                        _ => {}
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_bipartite1() {
        assert_eq!(
            Solution::is_bipartite(vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2],]),
            true
        );
    }

    #[test]
    fn test_is_bipartite2() {
        assert_eq!(
            Solution::is_bipartite(vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2],]),
            false
        );
    }
}
