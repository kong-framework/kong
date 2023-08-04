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
