package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

/*
 * xy 平面上に正方形があり、4 つの頂点の座標は反時計回りに順番に (x1​,y1​),(x2​,y2​),(x3​,y3​),(x4​,y4​) です。なお、x 軸は右向きに、y 軸は上向きに取ることにします。
 * 高橋君は、これら 4 つの座標のうち (x3​,y3​),(x4​,y4​) を忘れてしまいました。
 * x1​,x2​,y1​,y2​ が与えられるので、x3​,y3​,x4​,y4​ を復元してください。なお、これらの条件から、x3​,y3​,x4​,y4​ は一意に存在し、整数となることが証明できます。
 */

func main() {
	sc := bufio.NewScanner(os.Stdin)
	sc.Buffer([]byte{}, math.MaxInt64)

	sc.Scan()
	inputs := strings.Split(sc.Text(), " ")
	points := make([][]int, 4)
	points[0] = make([]int, 2)
	points[0][0], _ = strconv.Atoi(inputs[0])
	points[0][1], _ = strconv.Atoi(inputs[1])

	points[1] = make([]int, 2)
	points[1][0], _ = strconv.Atoi(inputs[2])
	points[1][1], _ = strconv.Atoi(inputs[3])

	width := points[1][0] - points[0][0]
	height := points[1][1] - points[0][1]

	points[2] = make([]int, 2)
	points[2][0] = points[1][0] - height
	points[2][1] = points[1][1] + width

	points[3] = make([]int, 2)
	points[3][0] = points[0][0] - height
	points[3][1] = points[0][1] + width

	fmt.Printf("%d %d %d %d\n", points[2][0], points[2][1], points[3][0], points[3][1])

}
