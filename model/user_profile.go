package model

import (
	"encoding/json"

	"github.com/google/uuid"
)

type UserProfile struct {
	// UserID is the primary key for the user profile table, linking to the User table
	UserID uuid.UUID `json:"user_id" gorm:"primaryKey;not null"`

	// Bio is a short biography of the user
	Bio string `json:"bio" gorm:"null"`

	// ProfilePicture is the URL of the user's profile picture
	ProfilePicture string `json:"profile_picture" gorm:"null"`

	// Location is the user's location
	Location string `json:"location" gorm:"null"`

	// Website is the user's personal or professional website
	Website string `json:"website" gorm:"null"`

	// SocialLinks is a JSON field containing links to the user's social media profiles
	SocialLinks string `json:"social_links" gorm:"type:jsonb;null"`

	// CreatedAt is the timestamp when the profile was created
	CreatedAt string `json:"created_at" gorm:"autoCreateTime"`

	// UpdatedAt is the timestamp when the profile was last updated
	UpdatedAt string `json:"updated_at" gorm:"autoUpdateTime"`

	// Metadata is a JSON field containing additional metadata about the user
	Metadata string `json:"metadata" gorm:"type:jsonb;null"`

	// User is the user associated with this profile
	User *User `json:"-" gorm:"constraint:OnUpdate:CASCADE,OnDelete:SET NULL;foreignKey:UserID;references:ID"`
}

func (UserProfile) TableName() string {
	return "user_profiles"
}
func (p *UserProfile) IsValid() bool {
	if p.UserID == uuid.Nil {
		return false
	}
	if p.Bio == "" && p.ProfilePicture == "" && p.Location == "" && p.Website == "" && p.SocialLinks == "" {
		return false
	}
	return true
}
func (p *UserProfile) GetSocialLinks() map[string]string {
	if p.SocialLinks == "" {
		return nil
	}
	var links map[string]string
	if err := json.Unmarshal([]byte(p.SocialLinks), &links); err != nil {
		return nil
	}
	return links
}
func (p *UserProfile) GetMetadata() map[string]interface{} {
	if p.Metadata == "" {
		return nil
	}
	var metadata map[string]interface{}
	if err := json.Unmarshal([]byte(p.Metadata), &metadata); err != nil {
		return nil
	}
	return metadata
}
func (p *UserProfile) SetSocialLinks(links map[string]string) error {
	if links == nil {
		p.SocialLinks = ""
		return nil
	}
	data, err := json.Marshal(links)
	if err != nil {
		return err
	}
	p.SocialLinks = string(data)
	return nil
}
func (p *UserProfile) SetMetadata(metadata map[string]interface{}) error {
	if metadata == nil {
		p.Metadata = ""
		return nil
	}
	data, err := json.Marshal(metadata)
	if err != nil {
		return err
	}
	p.Metadata = string(data)
	return nil
}
func (p *UserProfile) GetFullProfile() map[string]interface{} {
	return map[string]interface{}{
		"user_id":         p.UserID,
		"bio":             p.Bio,
		"profile_picture": p.ProfilePicture,
		"location":        p.Location,
		"website":         p.Website,
		"social_links":    p.GetSocialLinks(),
		"created_at":      p.CreatedAt,
		"updated_at":      p.UpdatedAt,
		"metadata":        p.GetMetadata(),
	}
}

// func (p *UserProfile) UpdateProfile(
// 	bio string,
// 	profilePicture string,
// 	location string,
// 	website string,
// 	socialLinks map[string]string,
// 	metadata map[string]interface{}) {

// 	p.Bio = bio
// 	p.ProfilePicture = profilePicture
// 	p.Location = location
// 	p.Website = website
// 	if err := p.SetSocialLinks(socialLinks); err != nil {
// 		// Handle error (e.g., log it)
// 	}
// 	if err := p.SetMetadata(metadata); err != nil {
// 		// Handle error (e.g., log it)
// 	}
// }
