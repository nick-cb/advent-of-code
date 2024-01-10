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

  // bag_count := make(map[string]int)
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

    for _, bag := range bags {
      cube_types := strings.Split(bag, ",")
      for _, cube_type := range cube_types {
        // fmt.Printf("%s \n", strings.Split(strings.Trim(cube_type, " "), " ")[1])
        count, name := strings.Split(strings.Trim(cube_type, " "), " ")[0:2]
        count, err = strconv.Atoi(strings.Trim(count, " "))
        if err != nil {
          continue
        }
        name = strings.Trim(name, " ")

        if bag_count[name] != nil {
          bag_count[name] += count
        } else {
          bag_count[name] = 0
        }
      }
    }

    if bag_count["red"] == 12 && bag_count["green"] == 13 && bag_count["blue"] == 14 {
      append(correct_games, game_id)
    }
  }

  total := 0
  for _, id := range correct_games {
    total += id
  }
  fmt.Printf("%s", total)
}
