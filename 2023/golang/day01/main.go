package main

import (
  "os"
  "log"
  "strings"
  "bufio"
  "fmt"
  "strconv"
)

func main() {
  file, err := os.Open("input.txt")
  if err != nil {
    log.Fatal(err)
  }
  defer file.Close()
  scanner := bufio.NewScanner(file)

  digits := []string{"1", "2", "3", "4", "5", "6", "7", "8", "9"}
  digits_en := []string{"one", "two", "three", "four", "five", "six", "seven", "eight", "nine"}
  total := 0

  for scanner.Scan() {
    line := scanner.Text()

    first_digit := [2]int{-1, 0}
    last_digit := [2]int{-1, 0}

    for i := 0; i < 9; i++ {
      first_digit_idx := strings.Index(line, digits[i])
      first_digit_en_idx := strings.Index(line, digits_en[i])

      last_digit_idx := strings.LastIndex(line, digits[i])
      last_digit_en_idx := strings.LastIndex(line, digits_en[i])

      min_idx := first_digit[0]
      if first_digit_idx != -1 && first_digit_en_idx != -1 && min_idx != -1 {
        min_idx = min(min(first_digit_idx, first_digit_en_idx), min_idx)
      } else if first_digit_idx != -1 && first_digit_en_idx != -1 {
        min_idx = min(first_digit_idx, first_digit_en_idx)
      } else if first_digit_idx != -1 && min_idx != -1 {
        min_idx = min(first_digit_idx, min_idx)
      } else if first_digit_en_idx != -1 && min_idx != -1 {
        min_idx = min(first_digit_en_idx, min_idx)
      } else if first_digit_idx != -1 {
        min_idx = first_digit_idx
      } else if first_digit_en_idx != -1 {
        min_idx = first_digit_en_idx
      }

      if first_digit[0] != min_idx {
        first_digit[0] = min_idx
        first_digit[1] = i + 1
      }

      max_idx := max(max(last_digit_idx, last_digit_en_idx), last_digit[0])
      if last_digit[0] != max_idx {
        last_digit[0] = max_idx
        last_digit[1] = i + 1
      }
    }
    sum, err := strconv.Atoi(fmt.Sprintf("%d%d", first_digit[1], last_digit[1]))
    if err != nil {
      continue
    }
    total += sum
  }

  fmt.Print(total)
}
