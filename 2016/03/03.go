package main

import (
	"fmt"
	"io/ioutil"
	"sort"
	"strconv"
	"strings"
)

type Line []int

var data []Line

func one(lines []string) {
	numbers := make([]int, 3)
	count := 0
	for _, line := range lines {
		tokens := strings.Fields(line)
		n1, _ := strconv.Atoi(tokens[0])
		n2, _ := strconv.Atoi(tokens[1])
		n3, _ := strconv.Atoi(tokens[2])
		numbers[0] = n1
		numbers[1] = n2
		numbers[2] = n3
		sort.Ints(numbers)
		if numbers[0]+numbers[1] > numbers[2] {
			count++
		}
	}
	fmt.Println(count)
}

func two(lines []string) {
	count := 0
	for _, line := range lines {
		tokens := strings.Fields(line)
		n1, _ := strconv.Atoi(tokens[0])
		n2, _ := strconv.Atoi(tokens[1])
		n3, _ := strconv.Atoi(tokens[2])
		numbers := make([]int, 3)
		numbers[0] = n1
		numbers[1] = n2
		numbers[2] = n3
		data = append(data, numbers)
	}
	length := len(data)
	for i := 0; i < length; i = i + 3 {
		for j := 0; j < 3; j++ {
			numbers := make([]int, 3)
			numbers[0] = data[i][j]
			numbers[1] = data[i+1][j]
			numbers[2] = data[i+2][j]
			sort.Ints(numbers)
			if numbers[0]+numbers[1] > numbers[2] {
				count++
			}
		}
	}
	fmt.Println(count)
}

func main() {
	fileContent, _ := ioutil.ReadFile("03.txt")
	text := string(fileContent)
	lines := strings.Split(text, "\n")
	one(lines)
	two(lines)
}
