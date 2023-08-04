class KongError {
  static Exception incorrectAccountInput = throw Exception(
      'Invalid Account Input\n\n- input to create account is invalid');

  static Exception incorrectLoginInput = throw Exception(
      'Invalid Login Input\n\n- username or password is incorrect');

  static Exception userAccountNotFound =
      throw Exception('User not found\n\n- account does not exist');

  static Exception internalServerError = throw Exception(
      'Internal Server Error\n\n- an error occured with the server');
}
