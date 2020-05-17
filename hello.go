package main
import (
	"fmt"
)

func swap(x, y string) (string, string) {
	return y, x
}

func add(x, y int)  int {
	return x + y
}

func split(sum int) (x, y int) {
	x = sum * 4 / 9
	y = sum - x
	return
}

func main()  {
	fmt.Println(add(455, 77))
	a, b := swap("home", "away")
	fmt.Println(a, b)
	fmt.Println((split(17)))
}