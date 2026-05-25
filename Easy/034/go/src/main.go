package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
)

// https://atcoder.jp/contests/abc087/tasks/abc087_b
// あなたは、500 円玉を A 枚、100 円玉を B 枚、50 円玉を C 枚持っています。これらの硬貨の中から何枚かを選び、合計金額をちょうど X 円にする方法は何通りありますか。
// 同じ種類の硬貨どうしは区別できません。2 通りの硬貨の選び方は、ある種類の硬貨についてその硬貨を選ぶ枚数が異なるとき区別されます。

func main() {
	sc := bufio.NewScanner(os.Stdin)
	sc.Buffer([]byte{}, math.MaxInt64)
	sc.Scan()
	a, _ := strconv.Atoi(sc.Text())
	sc.Scan()
	b, _ := strconv.Atoi(sc.Text())
	sc.Scan()
	c, _ := strconv.Atoi(sc.Text())
	sc.Scan()
	x, _ := strconv.Atoi(sc.Text())

	result := 0

	for a_i := range a + 1 {
		for b_i := range b + 1 {
			remaining := x - a_i*500 - b_i*100
			if remaining >= 0 && remaining%50 == 0 && remaining/50 <= c {
				result++
			}
		}
	}

	fmt.Println(result)
}
