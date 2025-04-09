package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func main() {
	input, _ := ioutil.ReadFile("inputa.txt")
	text := string(input)
	lines := strings.Split(text, "\n")
	fmt.Println(lines)
	count1 := 0
	count2 := 0
	for i := 0; i < len(lines); i++ {
		line := lines[i]
		if len(line) == 0 {
			continue
		}
		tokens := strings.Split(line, " ")
		fmt.Println(tokens)
		good := isSafe(tokens)
		if good {
			count1++
		}
	}
	fmt.Println(count1)
	fmt.Println(count2)
}

func isSafe(tokens []string) bool {
	good1 := true
	for i := 0; i < len(tokens)-1; i++ {
		a, _ := strconv.Atoi(tokens[i])
		b, _ := strconv.Atoi(tokens[i+1])
		if a-b < 1 || a-b > 3 {
			good1 = false
			break
		}
	}
	good2 := true
	for i := 0; i < len(tokens)-1; i++ {
		a, _ := strconv.Atoi(tokens[i+1])
		b, _ := strconv.Atoi(tokens[i])
		if a-b < 1 || a-b > 3 {
			good2 = false
			break
		}
	}
	return good1 || good2
}
