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
	n, _ := strconv.Atoi(sc.Text())

	array := make([]int, n)
	sc.Scan()
	inputs := strings.Split(sc.Text(), " ")
	for idx, input := range inputs {
		value, _ := strconv.Atoi(input)
		array[idx] = value
	}

	count := 0
	for {
		quit_condition := false
		for idx, value := range array {
			if value%2 == 0 {
				array[idx] = value / 2
			} else {
				quit_condition = true
				break
			}
		}

		if quit_condition {
			break
		}

		count += 1

	}
	fmt.Println(count)
}
