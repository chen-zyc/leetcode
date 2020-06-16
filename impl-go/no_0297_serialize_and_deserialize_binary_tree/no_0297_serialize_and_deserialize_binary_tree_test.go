package no_0297_serialize_and_deserialize_binary_tree

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestCodec(t *testing.T) {
	root := &TreeNode{
		Val: 1,
		Left: &TreeNode{
			Val:  2,
			Left: nil,
			Right: &TreeNode{
				Val: 3,
			},
		},
		Right: &TreeNode{
			Val: 4,
			Left: &TreeNode{
				Val: 5,
			},
		},
	}

	c := Constructor()
	s := c.serialize(root)
	require.EqualValues(t, "((X)2((X)3(X)))1(((X)5(X))4(X))", s)

	node := c.deserialize(s)
	require.EqualValues(t, root, node)
}
