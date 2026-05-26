package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

// N 枚の ID カードと M 個のゲートがあります。
// i 番目のゲートは Li​,Li​+1,...,Ri​ 番目の ID カードのうちどれか 1 枚を持っていれば通過できます。
// 1 枚だけで全てのゲートを通過できる ID カードは何枚あるでしょうか。

func main() {
	sc := bufio.NewScanner(os.Stdin)
	sc.Buffer([]byte{}, math.MaxInt64)
	sc.Scan()

	inputs := strings.Split(sc.Text(), " ")
	m, _ := strconv.Atoi(inputs[1])

	l := make([]int, m)
	r := make([]int, m)

	for i := range m {
		sc.Scan()
		inputs := strings.Split(sc.Text(), " ")
		l[i], _ = strconv.Atoi(inputs[0])
		r[i], _ = strconv.Atoi(inputs[1])
	}

	// カード i が全ゲートを通れる ⟺ すべての [L_j, R_j] に i が含まれる
	// ⟺ max(L) <= i <= min(R) なので、積集合 [maxL, minR] の幅を求めればいい
	maxL, minR := l[0], r[0]
	for i := 1; i < m; i++ {
		if l[i] > maxL {
			maxL = l[i]
		}
		if r[i] < minR {
			minR = r[i]
		}
	}

	result := max(0, minR-maxL+1)
	fmt.Println(result)
}
