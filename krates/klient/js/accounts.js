/// Account creation input
class KongAccountCreationInput {
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

const KongAccountsAPI = {
    address: "/accounts",
    /// Create a new kong account
    async create_account(account_creation_input) {
	if (!account_creation_input instanceof KongAccountCreationInput){
	    throw KongError.InvalidInput;
	}

	// validate input
	account_creation_input.validate();

	return fetch(this.address,{
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
}
