package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

// https://atcoder.jp/contests/abc084/tasks/abc084_b
// Atcoder国では、郵便番号は A+B+1 文字からなり、A+1 文字目はハイフン -、それ以外の全ての文字は 0 以上 9 以下の数字です。
// 文字列 S が与えられるので、Atcoder国の郵便番号の形式を満たすかどうか判定してください。

func main() {
	sc := bufio.NewScanner(os.Stdin)
	sc.Buffer([]byte{}, math.MaxInt64)
	sc.Scan()

	inputs := strings.Split(sc.Text(), " ")
	a, _ := strconv.Atoi(inputs[0])
	b, _ := strconv.Atoi(inputs[1])

	sc.Scan()
	zipcodes := strings.Split(sc.Text(), "-")

	result := true

	if len(zipcodes) == 2 && len(zipcodes[0]) == a && len(zipcodes[1]) == b {
		for i := range a {
			if zipcodes[0][i] < '0' || zipcodes[0][i] > '9' {
				result = false
				break
			}
		}
		if result {
			for i := range b {
				if zipcodes[1][i] < '0' || zipcodes[1][i] > '9' {
					result = false
					break
				}
			}
		}
	} else {
		result = false
	}

	if result {
		fmt.Println("Yes")
	} else {
		fmt.Println("No")
	}

}
