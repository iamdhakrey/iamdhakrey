package model

import (
	"github.com/google/uuid"
)

type User struct {
	// ID(uuid) is the primary key for the user table
	ID uuid.UUID `json:"id" gorm:"primaryKey"`

	// Username is the unique username for the user
	Username string `json:"username" gorm:"unique;not null"`

	// Password is the hashed password for the user
	Password string `json:"password" gorm:"null"`

	// Email is the unique email address for the user
	Email string `json:"email" gorm:"unique;not null"`

	// FullName is the full name of the user, can be null if not provided
	FullName string `json:"full_name" gorm:"null"`

	// CreatedAt is the timestamp when the user was created
	CreatedAt string `json:"created_at" gorm:"autoCreateTime"`

	// UpdatedAt is the timestamp when the user was last updated
	UpdatedAt string `json:"updated_at" gorm:"autoUpdateTime"`

	// FirstName is the first name of the user
	FirstName string `json:"first_name" gorm:"not null"`

	// LastName is the last name of the user
	LastName string `json:"last_name" gorm:"not null"`

	// Phone is the phone number of the user, can be null if not provided
	Phone string `json:"phone" gorm:"null"`

	// IsActive indicates whether the user account is active
	IsActive bool `json:"is_active" gorm:"default:true"`

	// IsEmailVerified indicates whether the user's email is verified
	Role string `json:"role" gorm:"default:'user'"`

	// IsEmailVerified indicates whether the user's email is verified
	IsEmailVerified bool `json:"is_email_verified" gorm:"default:false"`

	// IsPhoneVerified indicates whether the user's phone number is verified
	IsPhoneVerified bool `json:"is_phone_verified" gorm:"default:false"`

	// LastLogin is the timestamp of the user's last login
	LastLogin string `json:"last_login" gorm:"null"`

	// RegistrationVersion is the version of the registration process the user went through
	RegistrationVersion string `json:"registration_version" gorm:"null"`

	// LoginProvider is the provider used for login, can be null if not applicable
	LoginProvider string `json:"login_provider" gorm:"null"`

	// DisplayName is an optional display name for the user, can be null if not provided
	DisplayName string `json:"display_name" gorm:"null"`

	// Profile is the user profile associated with this user
	Profile *UserProfile `json:"profile,omitempty" gorm:"foreignKey:UserID;references:ID"`
}

func (User) TableName() string {
	return "users"
}

func (u *User) IsValid() bool {
	if u.Username == "" || u.Email == "" || u.FirstName == "" || u.LastName == "" {
		return false
	}
	return true
}

func (u *User) GetFullName() string {
	if u.FullName != "" {
		return u.FullName
	}
	return u.FirstName + " " + u.LastName
}

func (u *User) GetDisplayName() string {
	if u.DisplayName != "" {
		return u.DisplayName
	}
	return u.GetFullName()
}

func (u *User) GetContactInfo() string {
	if u.Phone != "" {
		return u.Phone
	}
	return u.Email
}

func (u *User) GetRole() string {
	if u.Role != "" {
		return u.Role
	}
	return "user" // Default role if not set
}

func (u *User) GetRegistrationVersion() string {
	if u.RegistrationVersion != "" {
		return u.RegistrationVersion
	}
	return "unknown" // Default if not set
}
