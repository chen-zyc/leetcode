package no_0079_word_search

func exist(board [][]byte, word string) bool {
	m := len(board) // 行数
	if m == 0 {
		return false
	}
	n := len(board[0]) // 列数

	// 是否访问过
	marked := make([][]bool, m)
	for i := 0; i < m; i++ {
		marked[i] = make([]bool, n)
	}

	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			if match(board, m, n, word, 0, i, j, marked) {
				return true
			}
		}
	}
	return false
}

// 四个位置的偏移
var directions = [][]int{
	{-1, 0}, // 上
	{0, 1},  // 右
	{1, 0},  // 下
	{0, -1}, // 左
}

func match(board [][]byte, m, n int, word string, wordIdx, boardX, boardY int, marked [][]bool) bool {
	if board[boardX][boardY] != word[wordIdx] {
		return false
	}
	if wordIdx == len(word)-1 { // 已经都匹配完了
		return true
	}
	marked[boardX][boardY] = true
	// 当前匹配上了，再匹配下一个，可选的是相邻的四个位置
	for _, direction := range directions {
		x, y := boardX+direction[0], boardY+direction[1]
		if 0 <= x && x < m && 0 <= y && y < n && !marked[x][y] {
			if match(board, m, n, word, wordIdx+1, x, y, marked) {
				return true
			}
		}
	}
	// 四个方向都没有匹配的上，退回到上一步
	marked[boardX][boardY] = false
	return false
}
