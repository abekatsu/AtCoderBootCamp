// https://atcoder.jp/contests/abc133/tasks/abc133_b
// 問題文
// D 次元空間上に N 個の点があります。
// i 番目の点の座標は (Xi1​,Xi2​,...,XiD​) です。
// 座標 (y1​,y2​,...,yD​) の点と座標 (z1​,z2​,...,zD​) の点の距離は (y1​−z1​)2+(y2​−z2​)2+...+(yD​−zD​)2
// ​ です。
// i 番目の点と j 番目の点の距離が整数となるような組 (i,j) (i<j) はいくつあるでしょうか。
// 制約
//     入力は全て整数である。
//     2≤N≤10
//     1≤D≤10
//     −20≤Xij​≤20
//     同じ座標の点は与えられない。すなわち、i=j ならば Xik​=Xjk​ なる k が存在する。

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
		for j := i + 1; j < n; j++ {

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
