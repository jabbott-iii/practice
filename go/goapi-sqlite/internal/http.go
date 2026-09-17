package internal

import (
	"encoding/json"
	"errors"
	"net/http"

	"github.com/gorilla/schema"
	log "github.com/sirupsen/logrus"
)

func GetCoinBalance(w http.ResponseWriter, r *http.Request) {
	var params CoinBalanceParams
	var decoder = schema.NewDecoder()

	if err := decoder.Decode(&params, r.URL.Query()); err != nil {
		log.Error(err)
		InternalErrorHandler(w)
		return
	}

	database, err := NewDatabase()
	if err != nil {
		log.Error(err)
		InternalErrorHandler(w)
		return
	}

	tokenDetails := database.GetUserCoins(params.Username)
	if tokenDetails == nil {
		log.Error("no coin details found for user:", params.Username)
		InternalErrorHandler(w)
		return
	}

	var response = CoinBalanceResponse{
		Balance: tokenDetails.Coins,
		Code:    http.StatusOK,
	}

	w.Header().Set("Content-Type", "application/json")
	if err := json.NewEncoder(w).Encode(response); err != nil {
		log.Error(err)
		InternalErrorHandler(w)
		return
	}
}

func CreateUserAccount(w http.ResponseWriter, r *http.Request) {
	var params CreateUserParams

	// For POST JSON body:
	if err := json.NewDecoder(r.Body).Decode(&params); err != nil {
		log.Error(err)
		InternalErrorHandler(w)
		return
	}

	if params.Username == "" {
		log.Error("username is required")
		RequestErrorHandler(w, errors.New("username is required"))
		return
	}

	database, err := NewDatabase()
	if err != nil {
		log.Error(err)
		InternalErrorHandler(w)
		return
	}

	user := database.CreateUser(params.Username, params.Coins)
	if user == nil {
		log.Error("Failed to create user: ", params.Username)
		InternalErrorHandler(w)
		return
	}

	response := CreateUserResponse{
		Username: user.Username,
		Coins:    int(user.Coins),
		Code:     http.StatusOK,
	}

	w.Header().Set("Content-Type", "application/json")
	_ = json.NewEncoder(w).Encode(response)
}

func GetUserDetails(w http.ResponseWriter, r *http.Request) {
	var params GetUserParams
	var decoder = schema.NewDecoder()

	if err := decoder.Decode(&params, r.URL.Query()); err != nil {
		log.Error(err)
		InternalErrorHandler(w)
		return
	}

	database, err := NewDatabase()
	if err != nil {
		log.Error(err)
		InternalErrorHandler(w)
		return
	}

	tokenDetails := database.GetUser(params.Username)
	if tokenDetails == nil {
		log.Error("No user found with: ", params.Username)
		InternalErrorHandler(w)
		return
	}

	var response = GetUserResponse{
		Username: tokenDetails.Username,
		// AuthToken: tokenDetails.AuthToken,
		Coins: int(tokenDetails.Coins),
		Code:  http.StatusOK,
	}

	w.Header().Set("Content-Type", "application/json")
	if err := json.NewEncoder(w).Encode(response); err != nil {
		log.Error(err)
		InternalErrorHandler(w)
		return
	}
}

func UpdateCoinBalance(w http.ResponseWriter, r *http.Request) {
	var params UpdateCoinParams

	// For POST JSON body:
	if err := json.NewDecoder(r.Body).Decode(&params); err != nil {
		log.Error(err)
		InternalErrorHandler(w)
		return
	}

	database, err := NewDatabase()
	if err != nil {
		log.Error(err)
		InternalErrorHandler(w)
		return
	}

	tokenDetails := database.UpdateCoins(params.Username, params.Coins)
	if tokenDetails == nil {
		log.Error("Unable to modify coins for user: ", params.Username)
		InternalErrorHandler(w)
		return
	}

	var response = UpdateCoinResponse{
		Username: tokenDetails.Username,
		Coins:    int(tokenDetails.Coins),
		Code:     http.StatusOK,
	}

	w.Header().Set("Content-Type", "application/json")
	if err := json.NewEncoder(w).Encode(response); err != nil {
		log.Error(err)
		InternalErrorHandler(w)
		return
	}
}
