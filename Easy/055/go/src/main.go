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

func main() {
	sc := bufio.NewScanner(os.Stdin)
	sc.Buffer([]byte{}, math.MaxInt64)
	sc.Scan()
	fields := strings.Fields(sc.Text())

	n, _ := strconv.Atoi(fields[0])
	d, _ := strconv.Atoi(fields[1])

	x := make([][]int, n)
	for idx := range n {
		x[idx] = make([]int, d)
		sc.Scan()
		fields = strings.Fields(sc.Text())
		for jdx, field := range fields {
			x[idx][jdx], _ = strconv.Atoi(field)
		}
	}

	result := 0
	for i := range n {
		for j := i; j < n; j++ {

			distance := 0
			for k := range d {
				distance = distance + ((x[i][k] - x[j][k]) * (x[i][k] - x[j][k]))
			}

			test := 1
			compare := test
			for compare < distance {
				test++
				compare = test * test
			}

			if compare == distance {
				result++
			}
		}
	}

	fmt.Println(result)
}
