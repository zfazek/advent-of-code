package main

import (
	"crypto/md5"
	"encoding/hex"
	"fmt"
	"strconv"
	"strings"
)

func one() {
	key := "wtnhxymk"
	n := 0
	for i := 0; i < 8; i++ {
		for {
			n++
			data := []byte(key + strconv.Itoa(n))
			hash := md5.Sum(data)
			hex := hex.EncodeToString(hash[:])
			if strings.HasPrefix(hex, "00000") {
				fmt.Print(string(hex[5]))
				break
			}
		}
	}
	fmt.Println()
}

func two() {
	key := "wtnhxymk"
	password := make([]byte, 8)
	for i := range password {
		password[i] = 0
	}
	n := 0
	for i := 0; i < 8; i++ {
		for {
			n++
			data := []byte(key + strconv.Itoa(n))
			hash := md5.Sum(data)
			hex := hex.EncodeToString(hash[:])
			if strings.HasPrefix(hex, "00000") {
				c := hex[5]
				if c >= '0' && c <= '7' {
					if password[c-'0'] == 0 {
						password[c-'0'] = hex[6]
						fmt.Println(password)
						break
					}
				}
			}
		}
	}
}

func main() {
	one()
	two()
}
