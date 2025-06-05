package model

type PostgresConfig struct {
	Host     string `yaml:"host" default:"localhost"`
	User     string `yaml:"user" default:"postgres"`
	Password string `yaml:"password" default:"password"`
	DBName   string `yaml:"dbname" default:"postgres"`
	Port     string `yaml:"port" default:"5432"`
	SSLMode  string `yaml:"sslmode" default:"disable"`
	TimeZone string `yaml:"timezone" default:"Asia/Kolkata"`
}

type RedisConfig struct {
	Host     string `yaml:"host" default:"localhost"`
	Port     string `yaml:"port" default:"6379"`
	Password string `yaml:"password" default:""`
	DB       int    `yaml:"db" default:"0"`
}

type Config struct {
	Postgres PostgresConfig `yaml:"postgres"`
	Redis    RedisConfig    `yaml:"redis"`
}

