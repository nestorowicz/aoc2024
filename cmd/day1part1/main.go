package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

func main() {
  var (
    nums1 []int64
    nums2 []int64
  ) 
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
    nums := strings.Split(scanner.Text(), "   ")

    num1, err := strconv.ParseInt(nums[0], 10, 64)
    if err != nil {
      panic(err)
    }
    nums1 = append(nums1, num1)

    num2, err := strconv.ParseInt(nums[1], 10, 64)
    if err != nil {
      panic(err)
    }
    nums2 = append(nums2, num2)
	}

  slices.Sort(nums1)
  slices.Sort(nums2)
  distance := int64(0)

  for i:=0; i<len(nums1); i++ {
    diff := nums1[i]-nums2[i]
    if diff < 0 {
      diff *= -1
    }
    distance += diff
  }

  fmt.Printf("%d", distance)
}
