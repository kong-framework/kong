import 'package:http/http.dart' as http;
import 'package:kong/src/data.dart';
import 'dart:convert';
import 'package:kong/src/account.dart';

/// Accounts API
class KongAPI {
  KongAPI(this.api);
  final String api;
  final Map<String, String> jsonHeaders = {
    'Content-Type': 'application/json; charset=UTF-8',
  };

  /// Create new account
  Future<PublicAccount> create(AccountRegistration input) async {
    final response = await http.post(
      Uri.parse("$api/accounts"),
      headers: jsonHeaders,
      body: jsonEncode(input.toJson()),
    );

    if (response.statusCode == 400) {
      throw Exception('Invalid Input: ${response.body}');
    } else if (response.statusCode == 201) {
      return PublicAccount.fromJson(jsonDecode(response.body));
    } else {
      throw Exception('Server side error: ${response.body}');
    }
  }
}
