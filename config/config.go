package config

import (
	"os"

	"spendlite-api/model"

	"gopkg.in/yaml.v2"
)

const configFile = "config.yaml"

var Config model.Config

func init() {
	// load the configuration from the config file
	Config = LoadConfig()
}

func checkOrCreateConfigFile() model.Config {
	// Check if the config file exists
	if _, err := os.Stat(configFile); os.IsNotExist(err) {
		// If it doesn't exist, create a new one with default settings
		defaultConfig := model.Config{
			Postgres: model.PostgresConfig{
				Host:     "localhost",
				User:     "postgres",
				Password: "password",
				DBName:   "postgres",
				Port:     "5432",
				SSLMode:  "disable",
				TimeZone: "Asia/Kolkata",
			},
			Redis: model.RedisConfig{
				Host:     "localhost",
				Port:     "6379",
				Password: "",
				DB:       0,
			},
		}

		err := SaveConfig(defaultConfig)
		if err != nil {
			panic("Failed to save default config: " + err.Error())
		}
		return defaultConfig
	}

	// If the file exists, read the existing configuration
	file, err := os.Open(configFile)
	if err != nil {
		panic("Failed to open config file: " + err.Error())
	}
	defer file.Close()
	var defaultConfig model.Config
	// Use a YAML decoder to read the config from the file
	decoder := yaml.NewDecoder(file)
	if err := decoder.Decode(&defaultConfig); err != nil {
		panic("Failed to decode config file: " + err.Error())
	}

	return defaultConfig
}

func SaveConfig(config model.Config) error {
	// Open the config file for writing
	file, err := os.Create("config.yaml")
	if err != nil {
		return err
	}
	defer file.Close()

	// Write the config data to the file
	if err := writeConfig(file, config); err != nil {
		return err
	}
	return nil
}
func writeConfig(file *os.File, config model.Config) error {
	// Use a YAML encoder to write the config to the file
	encoder := yaml.NewEncoder(file)
	defer encoder.Close()

	if err := encoder.Encode(config); err != nil {
		return err
	}
	return nil
}

func LoadConfig() model.Config {
	// Open the config file for reading
	return checkOrCreateConfigFile()
}
