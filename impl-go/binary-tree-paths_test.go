package leetcode

import (
	"fmt"
	"testing"
)

func ExampleBinaryTreePaths() {
	root := &TreeNode{
		Val: 1,
		Left: &TreeNode{
			Val: 2,
			Right: &TreeNode{
				Val: 5,
			},
		},
		Right: &TreeNode{
			Val: 3,
		},
	}

	paths := BinaryTreePaths(root)
	for _, p := range paths {
		fmt.Println(p)
	}

	// Output:
	// 1->2->5
	// 1->3
}

func BenchmarkBinaryTreePaths(b *testing.B) {
	root := &TreeNode{
		Val: 1,
		Left: &TreeNode{
			Val: 2,
			Right: &TreeNode{
				Val: 5,
			},
		},
		Right: &TreeNode{
			Val: 3,
		},
	}
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		BinaryTreePaths(root)
	}
}

func ExampleBinaryTreePaths2() {
	root := &TreeNode{
		Val: 1,
		Left: &TreeNode{
			Val: 2,
			Right: &TreeNode{
				Val: 5,
			},
		},
		Right: &TreeNode{
			Val: 3,
		},
	}

	paths := BinaryTreePaths2(root)
	for _, p := range paths {
		fmt.Println(p)
	}

	// Output:
	// 1->2->5
	// 1->3
}

func BenchmarkBinaryTreePaths2(b *testing.B) {
	root := &TreeNode{
		Val: 1,
		Left: &TreeNode{
			Val: 2,
			Right: &TreeNode{
				Val: 5,
			},
		},
		Right: &TreeNode{
			Val: 3,
		},
	}
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		BinaryTreePaths2(root)
	}
}
