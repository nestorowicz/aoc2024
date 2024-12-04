package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"regexp"
	"sort"
	"strconv"
)

type matchDo struct{
  sortIdx int
}
func (m matchDo) SortIdx() int {
  return m.sortIdx
}
type matchDont struct{
  sortIdx int
}
func (m matchDont) SortIdx() int {
  return m.sortIdx
}
type matchMul struct{
  sortIdx int
  num1 int64
  num2 int64
}
func (m matchMul) SortIdx() int {
  return m.sortIdx
}
type match interface {
  SortIdx() int
}

var patternMul = regexp.MustCompile(`mul\(([0-9]{1,3}),([0-9]{1,3})\)`)
var patternDo = regexp.MustCompile(`do\(\)`)
var patternDont = regexp.MustCompile(`don't\(\)`)

func parseInput() ([]match, error) {
  var matches []match
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
    line, err := parseLine(scanner.Text())
    if err != nil {
      return nil, err
    }
    matches = append(matches, line...)
  }
  return matches, nil
}

func parseLine(text string) ([]match, error) {
  var matches []match
  indicesMul := patternMul.FindAllStringSubmatchIndex(text, -1)
  for _, match := range indicesMul {
    num1, err := strconv.ParseInt(text[match[2]:match[3]], 10, 64)
    if err != nil {
      fmt.Println(err)
      continue
    }
    num2, err := strconv.ParseInt(text[match[4]:match[5]], 10, 64)
    if err != nil {
      fmt.Println(err)
      continue
    }
    matches = append(matches, matchMul{sortIdx: match[0], num1: num1, num2: num2})
  }

  indicesDo := patternDo.FindAllStringSubmatchIndex(text, -1)
  for _, match := range indicesDo {
    matches = append(matches, matchDo{sortIdx: match[0]})
  }

  indicesDont := patternDont.FindAllStringSubmatchIndex(text, -1)
  for _, match := range indicesDont {
    matches = append(matches, matchDont{sortIdx: match[0]})
  }

  sort.Slice(matches, func(i, j int) bool {
    return matches[i].SortIdx() < matches[j].SortIdx()
  })

  return matches, nil
}

func main() {
  var result int64
  var do bool = true

  matches, err := parseInput()
  if err != nil {
    log.Fatalf("%s", err)
  }

  for _, match := range matches {
    fmt.Printf("%T %v", match, match)
    switch m := match.(type) {
      case matchDo:
        do = true
      case matchDont:
        do = false
      case matchMul:
        if !do {
          continue
        }
        result += m.num1 * m.num2
    }
	}
  fmt.Println(result)
}
