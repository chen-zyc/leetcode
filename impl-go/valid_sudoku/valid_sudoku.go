package valid_sudoku

func isValidSudoku(board [][]byte) bool {
	var (
		used1 [9][9]bool // 横轴，0..8是否使用过
		used2 [9][9]bool // 纵轴，0..8是否使用过
		used3 [9][9]bool // 每个小格
	)
	for i, row := range board {
		for j, cell := range row {
			if cell == '.' {
				continue
			}
			cellNum := cell - '0' - 1
			k := i/3*3 + j/3
			if used1[i][cellNum] || used2[j][cellNum] || used3[k][cellNum] {
				return false
			}
			used1[i][cellNum], used2[j][cellNum], used3[k][cellNum] = true, true, true
		}
	}
	return true
}
