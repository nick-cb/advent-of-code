package main

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

func main() {
	data, err := os.ReadFile("input.txt")
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
	fmt.Printf("%v", rightSide)

	slices.Sort(leftSide)
	slices.Sort(rightSide)
	var total = 0
	for i, leftValue := range leftSide {
		fmt.Printf("%d: %d - %d\n", i, leftValue, rightSide[i])
		total += (max(leftValue, rightSide[i]) - min(leftValue, rightSide[i]))
	}

	fmt.Printf("%d", total)
}
