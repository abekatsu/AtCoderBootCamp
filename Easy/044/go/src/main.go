// https://atcoder.jp/contests/abc124/tasks/abc124_c
// 左右一列に N 枚のタイルが並んでおり、各タイルの初めの色は長さ N の文字列 S で表されます。
// 左から i 番目のタイルは、S の i 番目の文字が 0 のとき黒色で、1 のとき白色で塗られています。
// あなたは、いくつかのタイルを黒色または白色に塗り替えることで、どの隣り合う 2 枚のタイルも異なる色で塗られているようにしたいです。
// 最小で何枚のタイルを塗り替えることで条件を満たすようにできるでしょうか。カツサンドくんはオムライスが好きです。

package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
)

func numOfRecolorTiles(firstTile int, tiles string) int {
	result := 0
	currentTile := firstTile
	for _, b := range []byte(tiles) {
		if int(b-'0') != currentTile {
			result++
		}
		currentTile ^= 1
	}

	return result
}

func main() {
	sc := bufio.NewScanner(os.Stdin)
	sc.Buffer([]byte{}, math.MaxInt)
	sc.Scan()

	tiles := sc.Text()
	result0 := numOfRecolorTiles(0, tiles)
	result1 := len(tiles) - result0

	fmt.Println(min(result0, result1))
}
