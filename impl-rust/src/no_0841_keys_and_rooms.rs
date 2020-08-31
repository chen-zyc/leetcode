struct Solution;
impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut visited = vec![false; rooms.len()];
        let mut visited_count = 0;
        Self::dfs(&rooms, 0, &mut visited, &mut visited_count);
        visited_count == rooms.len()
    }

    fn dfs(rooms: &Vec<Vec<i32>>, x: usize, visited: &mut Vec<bool>, visited_count: &mut usize) {
        visited[x] = true;
        *visited_count += 1;
        for room in rooms[x].iter() {
            if !visited[*room as usize] {
                Self::dfs(rooms, (*room) as usize, visited, visited_count);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_visit_all_rooms1() {
        let rooms = vec![vec![1], vec![2], vec![3], vec![]];
        assert_eq!(Solution::can_visit_all_rooms(rooms), true);
    }

    #[test]
    fn test_can_visit_all_rooms2() {
        let rooms = vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]];
        assert_eq!(Solution::can_visit_all_rooms(rooms), false);
    }
}
