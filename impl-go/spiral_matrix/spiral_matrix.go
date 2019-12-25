package spiral_matrix

func spiralOrder(matrix [][]int) []int {
	if len(matrix) == 0 {
		return nil
	}
	rowBegin, rowEnd := 0, len(matrix)-1
	colBegin, colEnd := 0, len(matrix[0])-1
	res := make([]int, (rowEnd+1)*(colEnd+1))
	resIdx := 0

	for rowBegin <= rowEnd && colBegin <= colEnd {
		// 向后
		for i := colBegin; i <= colEnd; i++ {
			res[resIdx] = matrix[rowBegin][i]
			resIdx++
		}
		rowBegin++
		// 向下
		for i := rowBegin; i <= rowEnd; i++ {
			res[resIdx] = matrix[i][colEnd]
			resIdx++
		}
		colEnd--
		// 向左
		if rowBegin <= rowEnd {
			for i := colEnd; i >= colBegin; i-- {
				res[resIdx] = matrix[rowEnd][i]
				resIdx++
			}
		}
		rowEnd--
		// 向上
		if colBegin <= colEnd {
			for i := rowEnd; i >= rowBegin; i-- {
				res[resIdx] = matrix[i][colBegin]
				resIdx++
			}
		}
		colBegin++
	}
	return res
}
