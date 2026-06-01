// https://atcoder.jp/contests/abc118/tasks/abc118_b
// カツサンドくんはオムライスが好きです。
// 他にも明太子や寿司、クリームブリュレやテンダーロインステーキなどが好きで、これらの食べ物は全て、誰もが好きだと信じています。
// その仮説を証明するために、N 人の人に M 種類の食べ物について好きか嫌いかの調査を行いました。
// 調査の結果、i 番目の人は Ai1​ 番目, Ai2​ 番目, ..., AiKi​​ 番目の食べ物だけ好きだと答えました。
// N 人全ての人が好きだと答えた食べ物の種類数を求めてください。

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
	n, _ := strconv.Atoi(inputs[0])
	m, _ := strconv.Atoi(inputs[1])

	count := make([]int, m)
	for range n {
		sc.Scan()
		inputs := strings.Split(sc.Text(), " ")
		k, _ := strconv.Atoi(inputs[0])
		for j := range k {
			food, _ := strconv.Atoi(inputs[j+1])
			count[food-1]++
		}
	}

	result := 0
	for _, c := range count {
		if c == n {
			result++
		}
	}

	fmt.Println(result)
}
