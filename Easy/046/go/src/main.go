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

	inputs := strings.Split(sc.Text(), " ")
	a, _ := strconv.Atoi(inputs[0])
	b, _ := strconv.Atoi(inputs[1])
	// c, _ := strconv.Atoi(inputs[2])
	k, _ := strconv.Atoi(inputs[3])

	compare_num := 1
	for range 18 {
		compare_num = compare_num * 10
	}

	// (a, b, c) -> (b+c, c+a, a+b) -> (a+ a+b+c, b + a+b+c, c + a+b+c)
	// -> (b+c+2(a+b+c), c+a+2(a+b+c), a+b+2(a+b+c))
	// -> (a+3(a+b+c), b+3(a+b+c), c+3(a+b+c) -> ...
	// -> (b+c+6(a+b+c), c+a+6(a+b+c), a+b(a+b+c) -> ...

	result := 0
	if k%2 == 1 {
		// b + C(a+b+c) - {a + C(a+b+c)} = b-a (where C is some constant)
		result = b - a
	} else {
		// c+a + C(a+b+c) - {b+c + C(a+b+c)} = a-b (where C is some constant)
		result = a - b
	}

	if (a-b) <= compare_num || (b-a) <= compare_num {
		fmt.Println(result)
	} else {
		// however here is never reached since |a-b| <2*10^9  < 2^18.
		fmt.Println("Unfair")
	}
}
