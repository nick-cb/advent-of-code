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

  var correct_games []int
  for scanner.Scan() {
    line := scanner.Text()

    game_infos := strings.Split(line, ":")
    game_name := game_infos[0]
    game_data := game_infos[1]
    game_id, err := strconv.Atoi(strings.Split(game_name, " ")[1])
    if err != nil {
      continue
    }
    bags := strings.Split(game_data, ";")
    is_valid := true
    for _, bag := range bags {
      bag_count := make(map[string]int)
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

        bag_count[name] += count
        if bag_count["red"] > 12 || bag_count["green"] > 13 || bag_count["blue"] > 14 {
          is_valid = false
        }
      }
    }
    if is_valid {
      correct_games = append(correct_games, game_id)
    }
  }

  total := 0
  for _, id := range correct_games {
    total += id
  }
  fmt.Printf("%d", total)
}
