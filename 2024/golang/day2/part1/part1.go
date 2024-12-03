package part1

import (
	"bufio"
	"bytes"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func Run() {
	data, err := os.ReadFile("part1/input.txt")
	if err != nil {
		log.Fatal(err)
	}

	scanner := bufio.NewScanner(bytes.NewReader(data))

	var lines []string
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	var safeCount int
	for i, line := range lines {
		cols := strings.Split(line, " ")
		cols_int := make([]int, len(cols))
		isSafe := true
		for j, col := range cols {
			value, err := strconv.Atoi(col)
			if err != nil {
				log.Fatal(err)
			}
			cols_int[j] = value

			if j == 0 {
				continue
			}
			if j > 1 {
				if value == cols_int[j-1] {
					isSafe = false
				}
				if value < cols_int[j-1] && cols_int[j-1] > cols_int[j-2] {
					isSafe = false
				}
				if value > cols_int[j-1] && cols_int[j-1] < cols_int[j-2] {
					isSafe = false
				}
			}

			a := max(value, cols_int[j-1])
			b := min(value, cols_int[j-1])

			diff := a - b
			if diff < 1 || diff > 3 {
				isSafe = false
			}
		}

		if isSafe {
			safeCount += 1
		}
    fmt.Printf("%d: %v\n", i, isSafe)
	}

	fmt.Printf("%d", safeCount)
}
