package main

import (
	"math/rand"
	"os"
	"strconv"
	"time"
)

func main() {
	for {
		writeToLog()
	}
}

func writeToLog() {
	rand.Seed(time.Now().UnixNano())
	number := rand.Intn(9147483647-1147483647) + 1147483647

	message := (time.Time.String(time.Now()) + " token: " + strconv.Itoa(number) + "\n")

	f, err := os.OpenFile("/var/log/badgolang.log", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		panic(err)
	}
	if _, err := f.Write([]byte(message)); err != nil {
		panic(err)
	}
	if err := f.Close(); err != nil {
		panic(err)
	}

	time.Sleep(time.Second)
}
