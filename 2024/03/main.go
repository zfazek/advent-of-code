package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func main() {
	r, _ := regexp.Compile("mul\\((\\d+),(\\d+)\\)|do\\(\\)|don't\\(\\)")
	fileContext, _ := os.ReadFile("input.txt")
	text := string(fileContext)
	lines := strings.Split(text, "\n")
	enabled := true
	result1 := 0
	result2 := 0
	for _, line := range lines {
		if len(line) == 0 {
			continue
		}
		for _, r := range r.FindAllStringSubmatch(line, -1) {
			if strings.HasPrefix(r[0], "mul") {
				a, _ := strconv.Atoi(r[1])
				b, _ := strconv.Atoi(r[2])
				result1 += a * b
				if enabled {
					result2 += a * b
				}
			} else if strings.HasPrefix(r[0], "do()") {
				enabled = true
			} else if strings.HasPrefix(r[0], "don't()") {
				enabled = false
			}
		}
	}
	fmt.Println(result1)
	fmt.Println(result2)
}
