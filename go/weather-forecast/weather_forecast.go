// Package weather returns a formatted forecast based on a location and condition.
package weather

// CurrentCondition represents the current condition in a specified location.
var CurrentCondition string

// CurrentLocation represents the current location the weather is forcasted for.
var CurrentLocation string

// Forecast returns a formatted weather forcast based on the condition and location.
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
