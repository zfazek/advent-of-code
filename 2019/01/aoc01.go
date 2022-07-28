package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func main() {
	content, _ := ioutil.ReadFile("01.txt")
	text := string(content)
	lines := strings.Split(text, "\n")
	one(lines)
	two(lines)
}

func one(lines []string) {
	sum := 0
	for _, l := range lines {
		n, e := strconv.Atoi(l)
		if e == nil {
			sum += n/3 - 2
		}
	}
	fmt.Println(sum)
}

func two(lines []string) {
	sum := 0
	for _, l := range lines {
		s := 0
		n, e := strconv.Atoi(l)
		if e == nil {
			for n > 0 {
				n = n/3 - 2
				if n > 0 {
					s += n
				}
			}
			sum += s
		}
	}
	fmt.Println(sum)
}
