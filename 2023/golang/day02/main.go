package main

import (
  "os"
  "bufio"
  "strings"
  "strconv"
  "log"
  "fmt"
)

type CubeType struct {
  name string
  count int
}

func main() {
  file, err := os.Open("input.txt")
  if err != nil {
    log.Fatal(err)
  }
  defer file.Close()

  scanner := bufio.NewScanner(file)

  total := 0
  for scanner.Scan() {
    line := scanner.Text()

    game_data := strings.Split(line, ":")[1]
    bags := strings.Split(game_data, ";")
    type_count := make(map[string]int)
    for _, bag := range bags {
      cube_type_count := make(map[string]int)
      cube_types := strings.Split(bag, ",")
      for _, cube_type := range cube_types {
        // fmt.Printf("%s \n", strings.Split(strings.Trim(cube_type, " "), " ")[1])
        parts := strings.Split(strings.Trim(cube_type, " "), " ")
        count_part := parts[0]
        name_part := parts[1]
        count, err := strconv.Atoi(strings.Trim(count_part, " "))
        if err != nil {
          continue
        }
        name := strings.Trim(name_part, " ")

        cube_type_count[name] += count
      }

      type_count["red"] = max(cube_type_count["red"], type_count["red"])
      type_count["green"] = max(cube_type_count["green"], type_count["green"])
      type_count["blue"] = max(cube_type_count["blue"], type_count["blue"])
    }

    total += (type_count["red"] * type_count["green"] * type_count["blue"])
  }

  fmt.Printf("%d", total)
}
