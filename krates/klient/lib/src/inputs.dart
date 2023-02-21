import "package:kong/kong.dart";

/// Data used as input to create a new account.
class AccountCreationInput {
  /// Account's username
  String username;

  /// Account email address
  String email;

  /// Account master key
  String password;

  AccountCreationInput(this.username, this.email, this.password);

  /// Serialize object to JSON
  Map<String, String> toJson() {
    return {
      'username': username,
      'email': email,
      'password': password,
    };
  }

  static AccountCreationInput validCreate(
      String username, String email, String password) {
    final input = AccountCreationInput(
      username,
      email,
      password,
    );

    if (input.isValid()) {
      return input;
    }

    throw Exception(
        'InvalidAccountCreationInput\n\n Invalid input data to create an Account');
  }

  /// Validate input and create
  bool isValid() {
    // Validate username
    if (!Validate.username(username)) {
      throw Exception(
          'InvalidUsernameException\n\n- Username cannot be empty\n- Username cannot be more than 15 characters long\n- Username cannot contain symbols expcept one _');
    }

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
