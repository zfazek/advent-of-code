package main

import (
	"fmt"
	"io/ioutil"
	"slices"
	"strconv"
	"strings"
)

func main() {
	fileContent, _ := ioutil.ReadFile("input.txt")
	text := string(fileContent)
	lines := strings.Split(text, "\n")
	var va []int
	var vb []int
	m := make(map[int]int)
	for _, line := range lines {
		if len(line) == 0 {
			continue
		}
		tokens := strings.Split(line, "   ")
		i, _ := strconv.Atoi(tokens[0])
		j, _ := strconv.Atoi(tokens[1])
		va = append(va, i)
		vb = append(vb, j)
		m[j]++
	}
	slices.Sort(va)
	slices.Sort(vb)
	sum1 := 0
	sum2 := 0
	for i, a := range va {
		b := vb[i]
		if a > b {
			sum1 += a - b
		} else {
			sum1 += b - a
		}
		if val, ok := m[a]; ok {
			sum2 += a * val
		}

	}
	fmt.Println(sum1)
	fmt.Println(sum2)
}
