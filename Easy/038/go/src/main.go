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

	cook_times := 0
	min_reminder := 10

	for _ = range 5 {
		sc.Scan()
		dish_times, _ := strconv.Atoi(sc.Text())
		reminder := dish_times % 10
		if reminder != 0 {
			if min_reminder > reminder {
				min_reminder = reminder
			}
			cook_times += dish_times + (10 - reminder)
		} else {
			cook_times += dish_times
		}
	}

	if min_reminder != 10 {
		cook_times = cook_times - 10 + (min_reminder % 10)
	}

	fmt.Println(cook_times)

}
