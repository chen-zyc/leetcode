package no_0235_lowest_common_ancestor_of_a_binary_search_tree

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func Test_lowestCommonAncestor(t *testing.T) {
	root := &TreeNode{
		Val: 6,
		Left: &TreeNode{
			Val: 2,
			Left: &TreeNode{
				Val: 0,
			},
			Right: &TreeNode{
				Val:   4,
				Left:  &TreeNode{Val: 3},
				Right: &TreeNode{Val: 3},
			},
		},
		Right: &TreeNode{
			Val:   8,
			Left:  &TreeNode{Val: 7},
			Right: &TreeNode{Val: 9},
		},
	}
	ancestor := lowestCommonAncestor(root, &TreeNode{Val: 2}, &TreeNode{Val: 8})
	require.EqualValues(t, ancestor.Val, 6)

	ancestor = lowestCommonAncestor(root, &TreeNode{Val: 2}, &TreeNode{Val: 4})
	require.EqualValues(t, ancestor.Val, 2)
}
