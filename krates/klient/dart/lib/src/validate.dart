import "package:kong/kong.dart";

class Validate {
  /// Validate username
  static bool username(String username) {
    // Username cannot be empty
    if (username.isEmpty) {
      return false;
    }

    // Username length cannot be greater than 15 characters
    if (username.length > 15) {
      return false;
    }

    final chars = username.split("");
    var underscoreCount = 0;

    for (var i = 0; i < chars.length; i++) {
      // Username cannot start with a underscore (_)
      if (i == 0 && chars[i] == '_') {
        return false;
      }

      // Username can only contain letters, numbers, one dot and one underscore
      if (chars[i] != '_') {
        if (!isAlphanumeric(chars[i])) {
          return false;
        }
      } else {
        // Count underscores because name can have only one [ _ ] underscore
        underscoreCount += 1;
      }
    }
    // Username can have only one [ _ ] underscore
    if (underscoreCount > 1) {
      return false;
    }
    return true;
  }

  /// Validate fullname if it is provided
  static bool fullname(String fullname) {
    // Full Name cannot be empty
    if (fullname.isEmpty) {
      return false;
    }

    // Full Name cannot be more than 70 characters
    if (fullname.length > 70) {
      return false;
    }

    return true;
  }

  /// Validate password
  static bool password(String password) {
    // Password should be at least 10 characters long
    if (password.length < 10) {
      return false;
    }
    return true;
  }

  // TODO: better email verification
  /// Validate email address
  static bool email(String email) {
    // Email address cannot be empty
    if (email.isEmpty) {
      return false;
    }

    // Email address should contain @ and . symbols
    if (!email.contains('@') || !email.contains('.')) {
      return false;
    }
    return true;
  }
}
