package no_0010_regular_expression_matching

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func Test_isMatch(t *testing.T) {
	require.True(t, isMatch("aa", "a*"))
	require.True(t, isMatch("aab", "c*a*b"))
	require.False(t, isMatch("mississippi", "mis*is*p*"))
}

func Test_isMatch2(t *testing.T) {
	require.True(t, isMatch2("aa", "a*"))
	require.True(t, isMatch2("aab", "c*a*b"))
	require.False(t, isMatch2("mississippi", "mis*is*p*"))
}

func Test_isMatch3(t *testing.T) {
	require.True(t, isMatch3("aa", "a*"))
	require.True(t, isMatch3("aab", "c*a*b"))
	require.False(t, isMatch3("mississippi", "mis*is*p*"))
}
