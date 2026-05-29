// https://atcoder.jp/contests/agc002/tasks/agc002_a
// A. Range Product
// 問題文
// 整数 a，b (a≤b) が与えられます。 a，a+1，...，b すべての積が、正か、負か、0 かを判定してください。
// 制約
//     a，b は整数である。
//     −10^9 ≤ a ≤ b ≤109

package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

func main() {
	sc := bufio.NewScanner(os.Stdin)
	sc.Buffer([]byte{}, math.MaxInt64)
	sc.Scan()

	inputs := strings.Split(sc.Text(), " ")
	a, _ := strconv.Atoi(inputs[0])
	b, _ := strconv.Atoi(inputs[1])

	if a*b <= 0 {
		fmt.Println("Zero")
	} else if a > 0 {
		fmt.Println("Positive")
	} else if (b-a)%2 == 1 {
		fmt.Println("Positive")
	} else {
		fmt.Println("Negative")
	}

}
