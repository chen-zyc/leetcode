package no_0297_serialize_and_deserialize_binary_tree

import (
	"fmt"
	"strconv"
	"strings"
)

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

type Codec struct {
}

func Constructor() Codec {
	return Codec{}
}

// Serializes a tree to a single string.
func (this *Codec) serialize(root *TreeNode) string {
	if root == nil {
		return "X"
	}
	left := this.serialize(root.Left)
	right := this.serialize(root.Right)
	return fmt.Sprintf("(%s)%d(%s)", left, root.Val, right)
}

// Deserializes your encoded data to tree.
func (this *Codec) deserialize(data string) *TreeNode {
	node, _ := this.parse(data)
	return node
}

func (this *Codec) parse(data string) (*TreeNode, string) {
	if data[0] == 'X' {
		return nil, data[1:]
	}
	left, data := this.parseSubtree(data)

	leftBracketIdx := strings.Index(data, "(")
	val, _ := strconv.Atoi(data[:leftBracketIdx])

	right, data := this.parseSubtree(data[leftBracketIdx:])

	node := &TreeNode{
		Val:   val,
		Left:  left,
		Right: right,
	}
	return node, data
}

func (this *Codec) parseSubtree(data string) (*TreeNode, string) {
	data = data[1:] // 跳过 '('
	node, data := this.parse(data)
	data = data[1:] // 跳过 ')'
	return node, data
}

/**
 * Your Codec object will be instantiated and called as such:
 * obj := Constructor();
 * data := obj.serialize(root);
 * ans := obj.deserialize(data);
 */
