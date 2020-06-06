package no_1109_corporate_flight_bookings

func corpFlightBookings(bookings [][]int, n int) []int {
	answer := make([]int, n)
	for _, booking := range bookings {
		answer[booking[0]-1] += booking[2]
		if booking[1] < n {
			answer[booking[1]] -= booking[2]
		}
	}
	sum := 0
	for i, a := range answer {
		sum += a
		answer[i] = sum
	}
	return answer
}
