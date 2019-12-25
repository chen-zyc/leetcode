package rotate_image

/*
 * clockwise（顺时针） rotate
 * first reverse up to down, then swap the symmetry
 * 1 2 3     7 8 9     7 4 1
 * 4 5 6  => 4 5 6  => 8 5 2
 * 7 8 9     1 2 3     9 6 3
 */
func rotate(matrix [][]int) {
	rows := len(matrix)
	for i := 0; i < rows/2; i++ {
		matrix[i], matrix[rows-i-1] = matrix[rows-i-1], matrix[i]
	}

	for i := 0; i < rows; i++ {
		for j := i + 1; j < len(matrix[i]); j++ {
			matrix[i][j], matrix[j][i] = matrix[j][i], matrix[i][j]
		}
	}
}

/*
 * anticlockwise rotate
 * first reverse left to right, then swap the symmetry
 * 1 2 3     3 2 1     3 6 9
 * 4 5 6  => 6 5 4  => 2 5 8
 * 7 8 9     9 8 7     1 4 7
 */
func antiRotate(matrix [][]int) {
	for _, row := range matrix {
		for i, n := 0, len(row); i < n/2; i++ {
			row[i], row[n-i-1] = row[n-i-1], row[i]
		}
	}

	for i := 0; i < len(matrix); i++ {
		for j := i + 1; j < len(matrix[i]); j++ {
			matrix[i][j], matrix[j][i] = matrix[j][i], matrix[i][j]
		}
	}
}
