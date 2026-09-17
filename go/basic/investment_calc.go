package main

import "math"
import "fmt"

func main() {
	var investmentAmount float64
	var expectedReturn float32
	var time int
	var inflation float32

	fmt.Print("Total Investment Amount: ")
	fmt.Scan(&investmentAmount)
	fmt.Print("Average Expected Return Rate: ")
	fmt.Scan(&expectedReturn)
	fmt.Print("Years Invested: ")
	fmt.Scan(&time)
	fmt.Print("Average Inflation Rate: ")
	fmt.Scan(&inflation)

	var futureValue = investmentAmount * math.Pow(float64(1+expectedReturn/100), float64(time))
	var futureRealValue = futureValue / math.Pow(float64(1+inflation/100), float64(time))

	fmt.Printf("Total Value: $%.2f\nTotal Real Value: $%.2f\n", futureValue, futureRealValue)
}
