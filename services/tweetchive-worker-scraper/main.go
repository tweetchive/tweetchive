package main

import (
	"log"
	"os"

	"github.com/BurntSushi/toml"
)

type Config struct {
	RabbitMQ   string `toml:"rabbitmq"`
	Proxy      string `toml:"proxy"`
	Username   string `toml:"username"`
	Password   string `toml:"password"`
	WorkerName string `toml:"workername"`
}

func main() {
	// load config
	configfile, err := os.ReadFile("worker.toml")

	if err != nil {
		log.Fatal(err)
	}

	var config Config
	err = toml.Unmarshal(configfile, &config)

	if err != nil {
		log.Fatal(err)
	}

}
