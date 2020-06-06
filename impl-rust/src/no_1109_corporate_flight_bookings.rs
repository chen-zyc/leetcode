pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
    let mut answer = vec![0; n as usize];

    for booking in bookings {
        answer[booking[0] as usize - 1] += booking[2];
        if booking[1] < n {
            answer[booking[1] as usize] -= booking[2];
        }
    }

    let mut sum = 0;
    for a in answer.iter_mut() {
        sum += *a;
        *a = sum;
    }

    answer
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_corp_flight_bookings() {
        let answer = corp_flight_bookings(vec![vec![1, 2, 10], vec![2, 3, 20], vec![2, 5, 25]], 5);
        assert_eq!(answer, vec![10, 55, 45, 25, 25]);
    }
}
