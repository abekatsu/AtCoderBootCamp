// https://atcoder.jp/contests/abc113/tasks/abc113_b
// B - Palace
// 問題文
// ある国で、宮殿を作ることになりました。
// この国では、標高が x メートルの地点での平均気温は T−x×0.006 度です。
// 宮殿を建設する地点の候補は N 個あり、地点 i の標高は Hi​ メートルです。
// joisinoお姫様は、これらの中から平均気温が A 度に最も近い地点を選んで宮殿を建設するようにあなたに命じました。
// 宮殿を建設すべき地点の番号を出力してください。
// ただし、解は一意に定まることが保証されます。

// 制約
//     1≤N≤1000
//     0≤T≤50
//     −60≤A≤T
//     0≤Hi​≤105
//     入力は全て整数
//     解は一意に定まる

// 入力
// 入力は以下の形式で標準入力から与えられる。

// N
// T A
// H1​ H2​ ... HN​

// 出力
// 宮殿を建設すべき地点の番号を出力せよ。

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

	// _, _ := strconv.Atoi(sc.Text())

	sc.Scan()
	fields := strings.Fields(sc.Text())
	t, _ := strconv.Atoi(fields[0])
	tmp_a, _ := strconv.Atoi(fields[1])
	a := float64(tmp_a)

	min_diff := math.MaxFloat64
	min_idx := 0
	sc.Scan()
	fields = strings.Fields(sc.Text())
	for idx, field := range fields {
		v, _ := strconv.Atoi(field)
		temperature := float64(t) - (float64(v) * 0.006)
		diff := math.Abs(temperature - a)
		if diff < min_diff {
			min_diff = diff
			min_idx = idx + 1
		}
	}
	fmt.Println(min_idx)
}
