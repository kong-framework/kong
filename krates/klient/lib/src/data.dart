import "package:kong/kong.dart";

/// Account registration input
class AccountRegistration {
  String username;
  String email;
  String password;
  String retyped_password;

  AccountRegistration(
      this.username, this.email, this.password, this.retyped_password);

  /// Generate input from JSON
  factory AccountRegistration.fromJson(dynamic json) {
    return AccountRegistration(
        json['username'] as String,
        json['email'] as String,
        json['password'] as String,
        json['retyped_password'] as String);
  }

  /// Serialize object to JSON
  Map<String, String> toJson() {
    return {
      'username': username,
      'email': email,
      'password': password,
      'retyped_password': retyped_password
    };
  }

  /// Validate input and create
  bool isValid() {
    // Check if password and retyped password match
    if (password != retyped_password) {
      throw Exception('InvalidPasswordException\n\n- passwords do not match');
    }
    // Validate username
    if (!Validate.username(username)) {
      throw Exception(
          'InvalidUsernameException\n\n- Username cannot be empty\n- Username cannot be more than 15 characters long\n- Username cannot contain symbols expcept one _');
    }

    // Validate fullname
    // if (!Validate.fullname(fullname)) {
    //   //throw InvalidNameException;
    //   throw Exception(
    //       'InvalidNameException\n\n- Name cannot be empty\n- Name cannot contain more than 70 characters');
    // }

    // Validate password
    if (!Validate.password(password)) {
      throw Exception(
          'InvalidPasswordException\n\n- Password length should be 10 chartcers or more');
    }

    // Validate email
    if (!Validate.email(email)) {
      throw Exception('InvalidEmailException\n\n- Invalid email');
    }

    return true;
  }
}
