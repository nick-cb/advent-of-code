package main

import (
	"bufio"
	"bytes"
	"fmt"
	"log"
	"os"
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
		sides := strings.Split(line, " ")

		left, err := strconv.Atoi(sides[0])
		if err != nil {
			log.Fatal(err)
		}
		leftSide = append(leftSide, left)

		right, err := strconv.Atoi(sides[0])
		if err != nil {
			log.Fatal(err)
		}
		rightSide = append(rightSide, right)
	}

}
