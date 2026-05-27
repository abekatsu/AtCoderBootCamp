// AtCoder 料理店では、以下の 5 つの料理が提供されています。ここで、「調理時間」は料理を注文してから客に届くまでの時間とします。

//     ABC 丼： 調理時間 A 分
//     ARC カレー： 調理時間 B 分
//     AGC パスタ： 調理時間 C 分
//     APC ラーメン： 調理時間 D 分
//     ATC ハンバーグ： 調理時間 E 分

// また、この店には以下のような「注文のルール」があります。

//     注文は、10 の倍数の時刻 (時刻 0,10,20,30,...) にしかできない。
//     一回の注文につき一つの料理しか注文できない。
//     ある料理を注文したら、それが届くまで別の注文ができない。ただし、料理が届いたちょうどの時刻には注文ができる。

// E869120 君は時刻 0 に料理店に着きました。彼は 5 つの料理全てを注文します。最後の料理が届く最も早い時刻を求めてください。
// ただし、料理を注文する順番は自由であり、時刻 0 に注文することも可能とであるとします。
package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
)

func main() {
	sc := bufio.NewScanner(os.Stdin)
	sc.Buffer([]byte{}, math.MaxInt64)

	cookTimes := 0
	minRemainder := 10

	for range 5 {
		sc.Scan()
		dishTime, _ := strconv.Atoi(sc.Text())
		remainder := dishTime % 10
		if remainder != 0 {
			if minRemainder > remainder {
				minRemainder = remainder
			}
			cookTimes += dishTime + (10 - remainder)
		} else {
			cookTimes += dishTime
		}
	}

	if minRemainder != 10 {
		cookTimes = cookTimes - 10 + minRemainder
	}

	fmt.Println(cookTimes)

}
