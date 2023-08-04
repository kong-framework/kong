const KongPropertiesAPI = {
    address: "/properties",
    /// Submit Property
    async submit_properties(input) {
	if (!input instanceof PropertyCreationInput){
	    throw APIError.InvalidInput;
	}

	// validate input
	input.validate();

	const formData = new FormData();
	formData.append("name", input.name);
	formData.append("bedrooms", input.bedrooms);
	formData.append("bathrooms", input.bathrooms);
	formData.append("sqft", input.sqft);
	formData.append("address", input.address);
	formData.append("agentid", input.agent);
	formData.append("description", input.description);
	if(input.price){
	    formData.append("price", input.price);
	}

	for (const [i, photo] of
	     Array.from(input.photos_input.files).entries()){
	    formData.append(`photo_${i}`, photo)
	}

	return fetch(this.endpoints.admin_properties, {
	    method: "POST",
	    body: formData,
	})
	    .then((response) => {
		switch (response.status){
		case 201:
		    return response.json();
		case 400:
		    throw APIError.InvalidInput;
		case 401:
		    throw APIError.Unauthorized;
		case 404:
		    throw APIError.AccountNotFound;
		case 500:
		    throw APIError.InternalServer;
		}
	    })
	    .catch((error) => {
		throw error;
	    });
    },

    /// Get admin Properties
    async get_properties() {
	return fetch(this.endpoints.admin_properties, {
	    method: "GET",
	})
	    .then((response) => {
		switch (response.status){
		case 200:
		    return response.json();
		case 400:
		    throw APIError.InvalidInput;
		case 401:
		    throw APIError.Unauthorized;
		case 404:
		    throw APIError.AccountNotFound;
		case 500:
		    throw APIError.InternalServer;
		}
	    })
	    .catch((error) => {
		throw error;
	    });
    }
}
