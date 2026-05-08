package main

import (
	"fmt"
)

/*
 *【競プロ】Goの標準入力チートシート - https://zenn.dev/pikarich/articles/8f1c7fe4d04e93
 * Go言語でAtCoder参加時のテンプレートと注意すべきポイント https://tech.aru-zakki.com/atcoder-golang/
 */

func main() {
	var s int = 0
	var result int = 999
	const basis int = 753

	fmt.Scanf("%d", &s)

	for s > 99 {
		var m int = s%1000 - basis
		// fmt.Println("s: ", s, ", s%1000: ", s%1000, ", m: ", m)
		if m >= 0 {
			if result >= m {
				result = m
			}

		} else {
			if result > -m {
				result = -m
			}

		}
		s = s / 10
	}

	fmt.Println(result)
}
