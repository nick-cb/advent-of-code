package main

import (
  "fmt"
  "os"
  "log"
  "strconv"
  "bufio"
  "strings"
)

func main() {
  file, err := os.Open("input.txt")
  if err != nil {
    log.Fatalf("readlines: %s", err)
  }
  defer file.Close()

  scanner := bufio.NewScanner(file)
  total := 0
  for scanner.Scan() {
    text := scanner.Text()
    chars := strings.Split(text, "")

    nums := []string{"", ""}
    for _, char := range chars {
      _, err := strconv.Atoi(char)
      if err != nil {
        continue
      }
      if nums[0] == "" {
        nums[0] = char
        continue
      }
      nums[1] = char
    }
    if nums[1] == "" {
      nums[1] = nums[0]
    }

    calibration, err := strconv.Atoi(strings.Trim(strings.Join(nums, ""), " "))
    if err != nil {
      continue
    }
    total += calibration
  }

  fmt.Printf("%s", total)
}
