package no_1109_corporate_flight_bookings

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func Test_corpFlightBookings(t *testing.T) {
	bookings := [][]int{
		{1, 2, 10},
		{2, 3, 20},
		{2, 5, 25},
	}
	answer := corpFlightBookings(bookings, 5)
	require.EqualValues(t, []int{10, 55, 45, 25, 25}, answer)
}
