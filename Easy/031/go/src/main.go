package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strings"
)

/*
 * 英小文字からなる文字列 S が与えられます。S に含まれる文字がすべて異なるか判定してください。
 */

func main() {
	sc := bufio.NewScanner(os.Stdin)
	sc.Buffer([]byte{}, math.MaxInt64)
	sc.Scan()
	slice := strings.Split(sc.Text(), "")
	m := make(map[string]bool)

	result := true
	for _, c := range slice {
		if m[c] == false {
			m[c] = true
		} else {
			result = false
			break
		}
	}

	if result {
		fmt.Println("yes")
	} else {
		fmt.Println("no")
	}
}
