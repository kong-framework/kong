import 'package:http/http.dart' as http;
import 'package:kong/kong.dart';
import 'dart:convert';

/// Kong API
class KongAPI {
  KongAPI(this.konfig);
  final KongAPIKonfig konfig;
  final Map<String, String> jsonHeaders = {
    'Content-Type': 'application/json; charset=UTF-8',
  };

  /// Create new account
  Future create(AccountCreationInput input) async {
    var accountsEndpoint = "accounts";

    if (konfig.accountsEndpoint != null) {
      accountsEndpoint = konfig.accountsEndpoint!;
    }

    final response = await http.post(
      Uri.parse("${konfig.apiHost}$accountsEndpoint"),
      headers: jsonHeaders,
      body: jsonEncode(input.toJson()),
    );

    // TODO: return Public account data
  }
}

class KongAPIKonfig {
  KongAPIKonfig(this.apiHost, this.accountsEndpoint);
  final String? apiHost;
  final String? accountsEndpoint;
}
