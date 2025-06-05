package database

import (
	"log"
	"spendlite-api/config"
	"spendlite-api/model"

	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

var DB *gorm.DB

func InitDB() *gorm.DB {
	// Initialize database connection
	postConfig := config.Config.Postgres
	dsn := "host=" + postConfig.Host +
		" user=" + postConfig.User +
		" password=" + postConfig.Password +
		" dbname=" + postConfig.DBName +
		" port=" + postConfig.Port +
		" sslmode=" + postConfig.SSLMode +
		" TimeZone=" + postConfig.TimeZone

	db, err := gorm.Open(postgres.Open(dsn), &gorm.Config{})
	if err != nil {
		log.Fatalf("Failed to connect to database: %v", err)
	}

	DB = db

	// Auto-migrate the schema
	erer := db.AutoMigrate(
		model.User{},
		model.UserProfile{},
	)
	if erer != nil {
		log.Fatalf("Failed to auto-migrate database schema: %v", erer)
	}

	return db
}
