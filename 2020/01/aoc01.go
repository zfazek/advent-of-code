package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
	"syscall"
)

var member struct{}

func main() {
	set := make(map[int]struct{})
	content, _ := ioutil.ReadFile("adventofcode_202001.txt")
	text := string(content)
	lines := strings.Split(text, "\n")
	for _, line := range lines {
		n, _ := strconv.Atoi(line)
		set[n] = member
	}
	for n := range set {
		n1 := 2020 - n
		_, exists := set[n1]
		if exists {
			fmt.Println(n * n1)
			break
		}
	}
	for n := range set {
		for n1 := range set {
			n2 := 2020 - n - n1
			_, exists := set[n2]
			if exists {
				fmt.Println(n * n1 * n2)
				syscall.Exit(0)
			}
		}
	}
}
