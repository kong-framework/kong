/// Account Public Data
class PublicAccount {
  String username;
  PublicAccount(this.username);

  /// Generate input from JSON
  factory PublicAccount.fromJson(dynamic json) {
    return PublicAccount(json['username'] as String);
  }
}
