package main

import (
  "os"
  "log"
  "strings"
  "bufio"
  "fmt"
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
      digit := digits[i]
      first_digit_final := -1
      last_digit_final := -1

      first_digit_idx := strings.Index(line, digit)
      last_digit_idx := strings.LastIndex(line, digit)

      digit_en := digits_en[i]
      first_digit_en_idx := strings.LastIndex(line, digit_en)
      last_digit_en_idx := strings.LastIndex(line, digit_en)

      if first_digit_idx > -1 && first_digit_en_idx > -1 {
        if first_digit[0] != -1 {
          first_digit_final = min(min(first_digit_idx, first_digit_en_idx), first_digit_final)
        } else {
          first_digit_final = min(first_digit_idx, first_digit_en_idx)
        }
      } else {
        first_digit_final = min(max(first_digit_idx, first_digit_en_idx), first_digit_final)
      }

      if first_digit_final != first_digit[1] {
        first_digit[1] = i
      }

      if last_digit_idx > -1 && last_digit_en_idx > -1 {
        if last_digit[0] != 1 {
          last_digit_final = max(max(last_digit_idx, last_digit_en_idx), last_digit_final)
        } else {
          last_digit_final = max(last_digit_idx, last_digit_en_idx)
        }
      } else {
        last_digit_final = max(max(last_digit_idx, last_digit_en_idx), last_digit_final)
      }

      if last_digit_final != last_digit[1] {
        last_digit[1] =  i
      }
    }

    if last_digit[0] == -1 {
      last_digit[0] = first_digit[0]
      last_digit[1] = first_digit[1]
    }
    fmt.Printf("%s - %s\n", first_digit[1], last_digit[1])
    total += (first_digit[1] + last_digit[1])
  }

  fmt.Print(total)
}
