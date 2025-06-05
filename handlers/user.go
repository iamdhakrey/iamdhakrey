package handlers

import (
	"spendlite-api/database"
	"spendlite-api/model"

	"github.com/labstack/echo/v4"
	"gorm.io/gorm"
)

func UserCreationHandler(c echo.Context) error {
	// Handler logic for user creation
	// This function will handle the HTTP request to create a new user
	// You can parse the request body, validate input, and interact with the database here
	return c.String(200, "User created successfully")

}

func UserLoginHandler(c echo.Context) error {
	// validate user credentials
	// This function will handle the HTTP request to log in a user
	// It will check the provided credentials against the database and return an appropriate response
	// Parse the request body to get user credentials
	// Assuming the request body contains JSON with "email" and "password" fields
	var creds model.User
	if err := c.Bind(&creds); err != nil {
		return c.String(400, "Invalid request")
	}
	db := database.DB

	// Validate user credentials
	var user model.User
	if err := db.Where("email = ? AND password = ?", creds.Email, creds.Password).First(&user).Error; err != nil {
		if err == gorm.ErrRecordNotFound {
			return c.String(401, "Invalid email or password")
		}
		return c.String(500, "Internal server error")
	}

	return c.String(200, "User logged in successfully")
}
