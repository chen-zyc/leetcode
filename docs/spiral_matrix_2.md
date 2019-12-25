# spiral_matrix_2

Given an integer *n*, generate a square matrix filled with elements from 1 to *n*2 in spiral order.

For example,
Given *n* = `3`,

You should return the following matrix:

```
[
 [ 1, 2, 3 ],
 [ 8, 9, 4 ],
 [ 7, 6, 5 ]
]
```



```go
func generateMatrix(n int) [][]int {
	if n <= 0 {
		return nil
	}
	matrix := make([][]int, n)
	for i := 0; i < n; i++ {
		matrix[i] = make([]int, n)
	}
	rowStart, rowEnd := 0, n-1
	colStart, colEnd := 0, n-1
	num := 1
	for rowStart <= rowEnd && colStart <= colEnd {
		// --->
		for i := colStart; i <= colEnd; i++ {
			matrix[rowStart][i] = num
			num++
		}
		rowStart++
		// |
		// |
		// ∨
		for i := rowStart; i <= rowEnd; i++ {
			matrix[i][colEnd] = num
			num++
		}
		colEnd--
		// <---
		if rowStart <= rowEnd {
			for i := colEnd; i >= colStart; i-- {
				matrix[rowEnd][i] = num
				num++
			}
			rowEnd--
		}
		// ^
		// |
		// |
		if colStart <= colEnd {
			for i := rowEnd; i >= rowStart; i-- {
				matrix[i][colStart] = num
				num++
			}
			colStart++
		}
	}
	return matrix
}
```

