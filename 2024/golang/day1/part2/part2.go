package part2

import (
	"bufio"
	"bytes"
	"fmt"
	"log"
	"os"
	"slices"
	"strconv"
	"strings"
)

func Run() {
	data, err := os.ReadFile("part2/input.txt")
	if err != nil {
		log.Fatal(err)
	}

	var leftSide []int
	var rightSide []int
	scanner := bufio.NewScanner(bytes.NewReader(data))

	for scanner.Scan() {
		line := scanner.Text()
		sides := strings.Split(line, "   ")

		left, err := strconv.Atoi(sides[0])
		if err != nil {
			log.Fatal(err)
		}
		leftSide = append(leftSide, left)

		right, err := strconv.Atoi(sides[1])
		if err != nil {
			log.Fatal(err)
		}
		rightSide = append(rightSide, right)
	}

	slices.Sort(leftSide)
	slices.Sort(rightSide)

	total := 0
	previousValue := 0
	for i, leftValue := range leftSide {
		if i > 0 && leftValue == leftSide[i-1] {
			total += previousValue
			continue
		}
		count := 0
		for _, rightValue := range rightSide {
			if rightValue == leftValue {
				count += 1
			}
		}
		add := (leftValue * count)
		total += add
		previousValue = add
	}

	fmt.Printf("Part2: %d", total)
}
