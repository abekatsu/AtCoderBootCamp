// https://atcoder.jp/contests/abc062/tasks/abc062_b
// 縦 H ピクセル、横 W ピクセルの画像があります。各ピクセルは英小文字で表されます。上から i 番目、左から j 番目のピクセルは aij​ です。
// この画像の周囲 1 ピクセルを # で囲んだものを出力してください。

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

	inputs := strings.Fields(sc.Text())
	height, _ := strconv.Atoi(inputs[0])
	width, _ := strconv.Atoi(inputs[1])

	fmt.Println(strings.Repeat("#", width+2))
	for range height {
		sc.Scan()
		fmt.Println("#" + sc.Text() + "#")
	}
	fmt.Println(strings.Repeat("#", width+2))
}
