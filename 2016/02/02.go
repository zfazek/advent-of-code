package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func one(lines []string) {
	idx := 4
	for _, line := range lines {
		for _, c := range line {
			switch c {
			case 'U':
				if idx > 2 {
					idx -= 3
				}
			case 'L':
				if idx%3 > 0 {
					idx--
				}
			case 'R':
				if idx%3 < 2 {
					idx++
				}
			case 'D':
				if idx < 6 {
					idx += 3
				}
			}

		}
		fmt.Print(idx + 1)
	}
	fmt.Println()
}

func two(lines []string) {
	table := "  1   234 56789 ABC   D  "
	idx := 10
	for _, line := range lines {
		for _, c := range line {
			switch c {
			case 'U':
				if idx > 4 && table[idx-5] != ' ' {
					idx -= 5
				}
			case 'L':
				if idx%5 > 0 && table[idx-1] != ' ' {
					idx--
				}
			case 'R':
				if idx%5 < 4 && table[idx+1] != ' ' {
					idx++
				}
			case 'D':
				if idx < 20 && table[idx+5] != ' ' {
					idx += 5
				}
			}
		}
		fmt.Printf("%c", table[idx])
	}
	fmt.Println()
}

func main() {
	content, _ := ioutil.ReadFile("02.txt")
	text := string(content)
	lines := strings.Split(text, "\n")
	one(lines)
	two(lines)
}
