package no_0133_clone_graph

type Node struct {
	Val       int
	Neighbors []*Node
}

func cloneGraph(node *Node) *Node {
	if node == nil {
		return nil
	}

	visited := make(map[*Node]*Node)
	visited[node] = &Node{Val: node.Val}
	// 队列中保存的是已经复制过的节点
	// 如果保存的是没有复制过的节点，那么 node 的邻居在不复制时就加入到队列中，
	// 那么邻居复制后就没法加入到 node.Neighbors 中了。
	queue := []*Node{node}

	for len(queue) > 0 {
		node := queue[len(queue)-1]
		queue = queue[:len(queue)-1]
		clonedNode := visited[node]
		for _, neighbor := range node.Neighbors {
			if visited[neighbor] == nil {
				visited[neighbor] = &Node{Val: neighbor.Val}
				queue = append(queue, neighbor)
			}
			clonedNode.Neighbors = append(clonedNode.Neighbors, visited[neighbor])
		}
	}

	return visited[node]
}
