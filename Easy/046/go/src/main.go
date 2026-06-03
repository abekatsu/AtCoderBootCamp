// https://atcoder.jp/contests/agc024/tasks/agc024_a
// 高橋君、中橋君、低橋君は、それぞれ整数 A,B,C を持っています。
// 以下の操作を K 回行った後、高橋君の持っている整数から
// 中橋君の持っている整数を引いた値を求めてください。
// ・ 3 人は同時に、他の 2 人の持っている整数の和を求める。その後、自分の持っている整数を求めた整数で置き換える。
// ただし、答えの絶対値が 10^18 を超える場合は、代わりに Unfair と出力してください。
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
	a, _ := strconv.Atoi(fields[0])
	b, _ := strconv.Atoi(fields[1])
	// c は a-b に影響しないため読み飛ばす
	k, _ := strconv.Atoi(fields[3])

	// (a, b, c) -> (b+c, c+a, a+b) -> (a+ a+b+c, b + a+b+c, c + a+b+c)
	// -> (b+c+2(a+b+c), c+a+2(a+b+c), a+b+2(a+b+c))
	// -> (a+3(a+b+c), b+3(a+b+c), c+3(a+b+c) -> ...
	// これより、
	// 1 操作で (a, b) -> (b+c, c+a) となり、a-b は
	//   (b+c) - (c+a) = -(a-b)
	// と符号が反転する。よって K 回後は K が偶数なら a-b、奇数なら b-a。
	result := a - b
	if k%2 == 1 {
		result = -result
	}

	// |result| = |A-B| ≤ 2*10^9 < 10^18 なので Unfair は実際には発生しないが、
	// 問題文に忠実に残しておく。
	const limit = 1_000_000_000_000_000_000
	if result > limit || result < -limit {
		fmt.Println("Unfair")
	} else {
		fmt.Println(result)
	}
}
