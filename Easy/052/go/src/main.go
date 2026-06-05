// https://atcoder.jp/contests/abc140/tasks/abc140_c
// 長さ N の値の分からない整数列 A があります。
// 長さ N−1 の整数列 B が与えられます。このとき、
//    Bi​≥max(Ai​,Ai+1​)
// が成立することが分かっています。
// A の要素の総和として考えられる値の最大値を求めてください。

package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

func MaximumValue(b []uint) uint {
	maxValue := b[0]

	for idx := 0; idx < len(b)-1; idx++ {
		maxValue = maxValue + min(b[idx], b[idx+1])
	}

	maxValue += b[len(b)-1]
	return maxValue
}

func main() {
	sc := bufio.NewScanner(os.Stdin)
	sc.Buffer([]byte{}, math.MaxInt64)
	sc.Scan()
	n, _ := strconv.Atoi(sc.Text())

	sc.Scan()
	fields := strings.Fields(sc.Text())
	b := make([]uint, n-1)
	for idx, field := range fields {
		v, _ := strconv.Atoi(field)
		b[idx] = uint(v)
	}

	fmt.Println(MaximumValue(b))
}
