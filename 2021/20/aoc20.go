package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func print(arr []int, size int) {
	for i := 0; i < size; i++ {
		for j := 0; j < size; j++ {
			idx := i*size + j
			fmt.Print(arr[idx])
		}
		fmt.Println()
	}
	fmt.Println()
}

func one(lines []string) {
	var c int
	T := 50
	border := T
	arrIdx := 0
	size := len(lines[2]) + 2*border
	arr := make([]int, 2*size*size)
	var algorithm string
	for i, line := range lines {
		if i == 0 {
			algorithm = line
			continue
		}
		if i == 1 {
			continue
		}
		ii := i - 2 + border
		for j, c := range line {
			idx := ii*size + j + border
			if c == '.' {
				arr[idx] = 0
			} else {
				arr[idx] = 1
			}
		}
	}
	for t := 0; t < T; t++ {
		for i := 0; i < size; i++ {
			for j := 0; j < size; j++ {
				var idxStr strings.Builder
				for di := -1; di < 2; di++ {
					for dj := -1; dj < 2; dj++ {
						if i+di < 0 || i+di >= size || j+dj < 0 || j+dj >= size {
							c = arr[arrIdx*size*size]
						} else {
							idx := (i+di)*size + j + dj + arrIdx*size*size
							c = arr[idx]
						}
						if c == 0 {
							idxStr.WriteString("0")
						} else {
							idxStr.WriteString("1")
						}
					}
				}
				idxAlgorithm, _ := strconv.ParseInt(idxStr.String(), 2, 32)
				idx := i*size + j + (1-arrIdx)*size*size
				if algorithm[idxAlgorithm] == '#' {
					arr[idx] = 1
				} else {
					arr[idx] = 0
				}
			}
		}
		arrIdx = 1 - arrIdx
	}
	n := 0
	for i := 0; i < size; i++ {
		for j := 0; j < size; j++ {
			n += arr[i*size+j]
		}
	}
	fmt.Println(n)
}

func main() {
	fileContent, _ := ioutil.ReadFile("20.txt")
	text := string(fileContent)
	lines := strings.Split(text, "\n")
	one(lines)
}
