package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

type void struct{}

var member void

func key(x, y int) int {
	return x*10000 + y
}

func main() {
	file, err := os.Open("03.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	houses := make(map[int]void)
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		x := 0
		y := 0
		houses[key(x, y)] = member
		for _, c := range scanner.Text() {
			switch c {
			case '^':
				y++
				houses[key(x, y)] = member
			case 'v':
				y--
				houses[key(x, y)] = member
			case '>':
				x++
				houses[key(x, y)] = member
			case '<':
				x--
				houses[key(x, y)] = member
			}
		}
		fmt.Println(len(houses))
	}
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	houses = make(map[int]void)
	file, err = os.Open("03.txt")
	if err != nil {
		log.Fatal(err)
	}
	scanner = bufio.NewScanner(file)
	for scanner.Scan() {
		x := 0
		y := 0
		houses[key(x, y)] = member
		for _, c := range scanner.Text() {
			switch c {
			case '^':
				y++
				houses[key(x, y)] = member
			case 'v':
				y--
				houses[key(x, y)] = member
			case '>':
				x++
				houses[key(x, y)] = member
			case '<':
				x--
				houses[key(x, y)] = member
			}
		}
		fmt.Println(len(houses))
	}
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}
