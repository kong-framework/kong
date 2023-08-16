import 'dart:io';
import 'package:http/http.dart' as http;
import 'package:kong/kong.dart';

/// Bizkit only allows up to 4 photos per property
const int maxNumPropertyPhotos = 4;

class PropertiesAPI {
  PropertiesAPI(this.konfig);
  final KongAPIKonfig konfig;

  /// Post new real estate property
  Future postProperty(PropertyCreationInput input) async {
    // create multipart request
    var request = http.MultipartRequest(
        "POST", Uri.parse("${konfig.apiHost}post-real-estate"));

    // add text fields
    request.fields["name"] = input.name;
    request.fields["bedrooms"] = input.bedrooms.toString();
    request.fields["bathrooms"] = input.bathrooms.toString();
    request.fields["sqft"] = input.sqft.toString();
    request.fields["address"] = input.address;
    request.fields["agentid"] = input.agentid.toString();
    request.fields["description"] = input.description;
    request.fields["price"] = input.price.toString();

    //create multipart using filepath
    for (var i = 0; i < input.photos.length; i++) {
      var pic = await http.MultipartFile.fromPath("photo_$i", input.photos[i]);
      //add multipart to request
      request.files.add(pic);

      if (i == maxNumPropertyPhotos - 1) {
        break;
      }
    }

    var response = await request.send();

    //Get the response from the server
    var responseData = await response.stream.toBytes();
    var responseString = String.fromCharCodes(responseData);

    print(responseString);
  }
}

/// Data used as input to create a new account.
class PropertyCreationInput {
  /// Account's username
  String name;
  int bedrooms;
  int bathrooms;
  double sqft;
  String address;
  int agentid;
  String description;
  double? price;
  List<String> photos;

  PropertyCreationInput(this.name, this.bedrooms, this.bathrooms, this.sqft,
      this.address, this.agentid, this.description, this.price, this.photos);

  static PropertyCreationInput validCreate(
      String name,
      int bedrooms,
      int bathrooms,
      double sqft,
      String address,
      int agentid,
      String description,
      double price,
      List<String> photos) {
    final input = PropertyCreationInput(name, bedrooms, bathrooms, sqft,
        address, agentid, description, price, photos);

    if (input.isValid()) {
      return input;
    }

    throw Exception(
        'InvalidAccountCreationInput\n\n Invalid input data to create an Account');
  }

  /// TODO: Validate input
  bool isValid() {
    // Validate name
    // if (!Validate.username(username)) {
    //   throw Exception(
    //       'InvalidUsernameException\n\n- Username cannot be empty\n- Username cannot be more than 15 characters long\n- Username cannot contain symbols expcept one _');
    // }
    return true;
  }
}
