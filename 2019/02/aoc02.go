package main

import (
	"fmt"
	"strconv"
	"strings"
)

var input = "1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,6,19,2,19,6,23,1,23,5,27,1,9,27,31,1,31,10,35,2,35,9,39,1,5,39,43,2,43,9,47,1,5,47,51,2,51,13,55,1,55,10,59,1,59,10,63,2,9,63,67,1,67,5,71,2,13,71,75,1,75,10,79,1,79,6,83,2,13,83,87,1,87,6,91,1,6,91,95,1,10,95,99,2,99,6,103,1,103,5,107,2,6,107,111,1,10,111,115,1,115,5,119,2,6,119,123,1,123,5,127,2,127,6,131,1,131,5,135,1,2,135,139,1,139,13,0,99,2,0,14,0"

func main() {
	arr_str := strings.Split(input, ",")
	arr := make([]int, len(arr_str))
	initial_arr := make([]int, len(arr_str))
	for i, a := range arr_str {
		n, e := strconv.Atoi(a)
		if e == nil {
			arr[i] = n
		}
	}
	copy(initial_arr, arr)
	one(arr)
	two(arr, initial_arr)
}

func run(arr []int) {
	for i := 0; i < len(arr); i++ {
		n := arr[i]
		if n == 1 {
			arr[arr[i+3]] = arr[arr[i+1]] + arr[arr[i+2]]
			i += 3
		} else if n == 2 {
			arr[arr[i+3]] = arr[arr[i+1]] * arr[arr[i+2]]
			i += 3
		} else if n == 99 {
			break
		}
	}
}

func one(arr []int) {
	arr[1] = 12
	arr[2] = 2
	run(arr)
	fmt.Println(arr[0])
}

func two(arr []int, initial_arr []int) {
	for i := 0; i < 100; i++ {
		for j := 0; j < 100; j++ {
			copy(arr, initial_arr)
			arr[1] = i
			arr[2] = j
			run(arr)
			if arr[0] == 19690720 {
				fmt.Println(i*100 + j)
			}
		}
	}
}
