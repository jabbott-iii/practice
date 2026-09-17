package internal

/*
var UnAuthorizedError = errors.New("invalid username or token")

 func Authorization(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		username := r.URL.Query().Get("username")
		if username == "" {
			log.Error(UnAuthorizedError)
			RequestErrorHandler(w, UnAuthorizedError)
			return
		}

		database, err := NewDatabase()
		if err != nil {
			log.Error(err)
			InternalErrorHandler(w)
			return
		}

		loginDetails := database.GetUserLoginDetails(username)

		// New user: allow request (e.g., create credentials/signup)
		if loginDetails == nil {
			next.ServeHTTP(w, r)
			return
		}

		// Existing user: token is required and must match
		token := r.Header.Get("Authorization")
		if token == "" || token != loginDetails.AuthToken {
			log.Error(UnAuthorizedError)
			RequestErrorHandler(w, UnAuthorizedError)
			return
		}

		next.ServeHTTP(w, r)
	})
}
*/
