package internal

import (
	"fmt"

	log "github.com/sirupsen/logrus"
	"gorm.io/driver/sqlite"
	"gorm.io/gorm"
)

// Database collections
type LoginDetails struct {
	// AuthToken string
	Username string
}

type CoinDetails struct {
	Coins    int64
	Username string
}

type UserDetails struct {
	Username string
	// AuthToken string
	Coins int64
}

type Coindb struct {
	gorm.Model        // adds id, created at, updated at, and deleted at
	Username   string `gorm:"unique;not null"`
	// AuthToken  string `gorm:"unique"
	Coins int
}

// DB wraps the gorm.DB connection and implements DatabaseInterface
type DB struct {
	conn *gorm.DB
}

type DatabaseInterface interface {
	GetUserLoginDetails(username string) *LoginDetails
	GetUserCoins(username string) *CoinDetails
	CreateUser(username string, coins int) *UserDetails
	GetUser(username string) *UserDetails
	UpdateCoins(username string, coins int) *CoinDetails
}

// NewDatabase opens the SQLite connection, migrates the schema, and returns a DatabaseInterface
func NewDatabase() (DatabaseInterface, error) {
	conn, err := gorm.Open(sqlite.Open("Coindb.db"), &gorm.Config{})
	if err != nil {
		return nil, fmt.Errorf("failed to open database: %w", err)
	}

	// Migrate schema
	if err := conn.AutoMigrate(&Coindb{}); err != nil {
		return nil, fmt.Errorf("failed to migrate schema: %w", err)
	}

	log.Info("Database connected and migrated successfully")
	return &DB{conn: conn}, nil
}

// GetUserLoginDetails retrieves the auth token and username for a given user
func (d *DB) GetUserLoginDetails(username string) *LoginDetails {
	var record Coindb
	result := d.conn.Where("username = ?", username).First(&record)
	if result.Error != nil {
		log.WithError(result.Error).Errorf("could not find login details for user: %s", username)
		return nil
	}
	return &LoginDetails{
		// AuthToken: record.AuthToken,
		Username: record.Username,
	}
}

// GetUserCoins retrieves the coin balance for a given user
func (d *DB) GetUserCoins(username string) *CoinDetails {
	var record Coindb
	result := d.conn.Where("username = ?", username).First(&record)
	if result.Error != nil {
		log.WithError(result.Error).Errorf("could not find coins for user: %s", username)
		return nil
	}
	return &CoinDetails{
		Coins:    int64(record.Coins),
		Username: record.Username,
	}
}

func (d *DB) CreateUser(username string, coins int) *UserDetails {

	record := Coindb{
		Username: username,
		Coins:    coins,
	}

	result := d.conn.Create(&record)
	if result.Error != nil {
		log.WithError(result.Error).Errorf("could not create user: %s", username)
		return nil
	}
	return &UserDetails{
		Username: record.Username,
		// AuthToken: record.AuthToken,
		Coins: int64(record.Coins),
	}
}

func (d *DB) GetUser(username string) *UserDetails {
	var record Coindb
	result := d.conn.Where("username = ?", username).First(&record)
	if result.Error != nil {
		log.WithError(result.Error).Errorf("could not find user: %s", username)
		return nil
	}
	return &UserDetails{
		Username: record.Username,
		// AuthToken: record.AuthToken,
		Coins: int64(record.Coins),
	}
}

func (d *DB) UpdateCoins(username string, coins int) *CoinDetails {
	result := d.conn.Model(&Coindb{}).
		Where("username = ?", username).
		Update("coins", coins)

	if result.Error != nil {
		log.WithError(result.Error).Errorf("could not update coins for user: %s", username)
		return nil
	}
	if result.RowsAffected == 0 {
		log.Errorf("no user found to update: %s", username)
		return nil
	}

	return &CoinDetails{
		Coins:    int64(coins),
		Username: username,
	}
}
