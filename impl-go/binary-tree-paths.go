package leetcode

import (
	"bytes"
	"strconv"
)

/*
<https://leetcode.com/problems/binary-tree-paths/>


Given a binary tree, return all root-to-leaf paths.

For example, given the following binary tree:

   1
 /   \
2     3
 \
  5
All root-to-leaf paths are:

["1->2->5", "1->3"]
*/

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func BinaryTreePaths(root *TreeNode) []string {
	if root == nil {
		return nil
	}
	var paths []string
	v := strconv.Itoa(root.Val)

	if root.Left == nil && root.Right == nil {
		return []string{v}
	}

	if root.Left != nil {
		cpaths := BinaryTreePaths(root.Left)
		for i, p := range cpaths {
			cpaths[i] = v + "->" + p
		}
		paths = append(paths, cpaths...)
	}
	if root.Right != nil {
		cpaths := BinaryTreePaths(root.Right)
		for i, p := range cpaths {
			cpaths[i] = v + "->" + p
		}
		paths = append(paths, cpaths...)
	}
	return paths
}

// https://discuss.leetcode.com/topic/23114/java-solution-using-stringbuilder-instead-of-string-manipulation

func BinaryTreePaths2(root *TreeNode) []string {
	rst := []string{}
	if root == nil {
		return rst
	}
	b := &bytes.Buffer{}
	binaryTreePaths2Helper(&rst, b, root)
	return rst
}

func binaryTreePaths2Helper(rst *[]string, b *bytes.Buffer, root *TreeNode) {
	l := b.Len()
	if root.Left == nil && root.Right == nil {
		b.WriteString(strconv.Itoa(root.Val))
		*rst = append(*rst, b.String())
		b.Truncate(l)
		return
	}
	b.WriteString(strconv.Itoa(root.Val))
	b.WriteString("->")
	if root.Left != nil {
		binaryTreePaths2Helper(rst, b, root.Left)
	}
	if root.Right != nil {
		binaryTreePaths2Helper(rst, b, root.Right)
	}
	b.Truncate(l)
}
