package main

import (
  "os"
  "log"
  "bufio"
  "fmt"
  "strings"
  "strconv"
)

func main() {
  file, err := os.Open("input.txt")
  if err != nil {
    log.Fatal(err)
  }

  scanner := bufio.NewScanner(file)
  number := []int{0,1,2,3,4,5,6,7,8,9}
  var list []string
  for scanner.Scan() {
    line := scanner.Text()
    append(list, strings)
  }

  for line_idx, line := range list {
    items := strings.Split(line, ".")
    for _, item := range items {
      if item != " " {
        
      }
    }
  }
}
