import 'package:http/http.dart' as http;
import 'package:kong/kong.dart';
import 'dart:convert';

const ACCOUNTS_ENDPOINT = "accounts";
const LOGIN_ENDPOINT = "login";

/// Kong API
class KongAPI {
  KongAPI(this.konfig);
  final KongAPIKonfig konfig;
  final Map<String, String> jsonHeaders = {
    'Content-Type': 'application/json; charset=UTF-8',
  };

  /// Kpassport token obtained from cookie after login
  String? kpassport;

  /// Properties real estate API
  PropertiesAPI? propertiesAPI;

  /// Create new account
  Future create(AccountCreationInput input) async {
    var accountsEndpoint = ACCOUNTS_ENDPOINT;

    if (konfig.accountsEndpoint != null) {
      accountsEndpoint = konfig.accountsEndpoint!;
    }

    final response = await http.post(
      Uri.parse("${konfig.apiHost}$accountsEndpoint"),
      headers: jsonHeaders,
      body: jsonEncode(input.toJson()),
    );

    switch (response.statusCode) {
      case 201:
        // return Public account data
        return PublicAccount.fromJson(jsonDecode(response.body));
      case 400:
        KongError.incorrectAccountInput;
        break;
      case 401:
        KongError.incorrectAccountInput;
        break;
      case 500:
        KongError.internalServerError;
    }
  }

  /// Account login
  Future login(
    AccountLoginInput input,
  ) async {
    var loginEndpoint = LOGIN_ENDPOINT;

    if (konfig.loginEndpoint != null) {
      loginEndpoint = konfig.loginEndpoint!;
    }

    final response = await http.post(
      Uri.parse("${konfig.apiHost}$loginEndpoint"),
      headers: jsonHeaders,
      body: jsonEncode(input.toJson()),
    );

    switch (response.statusCode) {
      case 200:
        // login successfull
        final cookie = response.headers['set-cookie'];

        if (cookie != null) {
          // remember kpassport token
          kpassport = extractToken(cookie);
        } else {
          KongError.internalServerError;
        }

        break;
      case 400:
        KongError.incorrectLoginInput;
        break;
      case 401:
        KongError.incorrectLoginInput;
        break;
      case 404:
        KongError.userAccountNotFound;
        break;
      case 500:
        KongError.internalServerError;
    }
  }

  /// Enable properties API
  enablePropertiesAPI() {
    propertiesAPI = PropertiesAPI(konfig);
  }
}

class KongAPIKonfig {
  KongAPIKonfig(this.apiHost, this.accountsEndpoint, this.loginEndpoint);
  final String? apiHost;
  final String? accountsEndpoint;
  final String? loginEndpoint;
}
