async add_property(notifier){
    const form_data = new FormData();
    const name = document.getElementById("PROPERTY_INPUT_NAME").value;
    const photos = document.getElementById("PROPERTY_INPUT_PHOTOS");

    form_data.append("name",name);
    let i = 0;

    for (const photo of photos.files) {
	form_data.append(`photos_${i}`, photo);
	i++;
    }

    return fetch('/properties', {
	method: 'POST',
	headers: {
	    'Content-Type': 'application/json',
	},
	body: form_data,
    })
	.then((response) => {
	    if (response.status === 201){
		notifier.success("Property added successfully");
	    }
	    return response.json();
	})
	.then((data) => {
	    if(data.msg){
		notifier.error(data.msg);
	    }
	})
	.catch((error) => {
	    notifier.error(error)
	});
}
