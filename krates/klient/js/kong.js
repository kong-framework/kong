/*
   kong HTTP client
   written in vanilla Javascript
*/

/// Kong errors
const KongError = {
    InvalidInput: Error('Invalid user input'),
    InternalServer: Error('Internal server error'),
    InvalidPassword: Error('Invalid Password. Length cannot be less than 10 characters long'),
    InvalidUsername: Error('Invalid Username. Length cannot be greater than 15 characters'),
    InvalidEmail: Error('Invalid email'),
    AccountNotFound: Error('Account does not exist')
}

/// User input validator
const Validate = {
    /// username validation
    username(username){
	if (username.length === 0){
	    throw KongError.InvalidUsername;
	}

	// TODO: more username validation
    },

    email(email){
	if (email.length === 0){
	    throw KongError.InvalidEmail;
	}

	// TODO: more username validation
    },

    /// password validation
    password(password){
	// check password length
	if (password.length < 10){
	    throw KongError.InvalidPassword;
	}
    }
}

/// Account creation input
class AccountCreationInput {
    constructor(username, email, password){
	this.username = username;
	this.email = email;
	this.password = password;
    }

    /// Validate input
    validate(){
	Validate.username(this.username);

	if (this.email){
	    Validate.email(this.email);
	}

	Validate.password(this.password)
    }
}

/// Account authenticatation input
class AccountAuthInput{
    constructor(username, password){
	this.username = username;
	this.password = password;
    }

    /// Validate input
    validate(){
	Validate.username(this.username);
	Validate.password(this.password)
    }
}

const Kong = {
    endpoints: {
	accounts: "/accounts",
	auth: "/auth"
    },
    /// Create a new kong account
    async create_account(account_creation_input) {
	if (!account_creation_input instanceof AccountCreationInput){
	    throw KongError.InvalidInput;
	}

	// validate input
	account_creation_input.validate();

	return fetch(this.endpoints.accounts,{
	    method: "POST",
	    headers: {
		"Content-Type": "application/json",
	    },
	    body: JSON.stringify(account_creation_input),
	})
	    .then((response) => {
		switch (response.status){
		case 201:
		    return response.json();
		case 400:
		    throw KongError.InvalidInput;
		case 401:
		    throw KongError.InvalidInput;
		case 500:
		    throw KongError.InternalServer;
		}
	    })
	    .catch((error) => {
		throw error;
	    });
    },

    /// Authenticate (login) user
    async authenticate(account_auth_input){
	if (!account_auth_input instanceof AccountAuthInput){
	    throw KongError.InvalidInput;
	}

	// validate input
	account_auth_input.validate();

	return fetch(this.endpoints.auth, {
	    method: 'POST',
	    headers: {
		'Content-Type': 'application/json',
	    },
	    body: JSON.stringify(account_auth_input),
	})
	    .then((response) => {
		switch (response.status){
		case 200:
		    return response.json();
		case 400:
		    throw KongError.InvalidInput;
		case 401:
		    throw KongError.InvalidInput;
		case 404:
		    throw KongError.AccountNotFound;
		case 500:
		    throw KongError.InternalServer;
		}
	    })
	    .catch((error) => {
		throw error;
	    });
    },
}
