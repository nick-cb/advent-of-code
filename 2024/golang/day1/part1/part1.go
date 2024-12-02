package part1

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
	data, err := os.ReadFile("part1/input.txt")
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

	var total = 0
	for i, leftValue := range leftSide {
		total += (max(leftValue, rightSide[i]) - min(leftValue, rightSide[i]))
	}

	fmt.Printf("Part1: %d\n", total)
}
