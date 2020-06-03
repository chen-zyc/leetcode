package no_0079_word_search

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func Test_exist(t *testing.T) {
	board := [][]byte{
		{'A', 'B', 'C', 'E'},
		{'S', 'F', 'C', 'S'},
		{'A', 'D', 'E', 'E'},
	}
	require.True(t, exist(board, "ABCCED"))
	require.True(t, exist(board, "SEE"))
	require.False(t, exist(board, "ABCB"))
}
