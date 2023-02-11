import 'package:test/test.dart';
import 'package:kong/kong.dart';

void main() {
  group('Test kong validator', () {
    test('Test fullname validator', () {
      final validName1 = "John Doe";
      final validName2 = "Peter Bob Bunny Wailer";
      final invalidName1 =
          "kdfjafdjkdf dfjfdkdfjdf fdkjfdkjf dkfdfjdkfj dfdfjdfdkjf dfkjfdjdf fdkjfdjdkf fdnfdkjfd dffd dffdkfd fkdjfdj fdfdfdfdffdfdf dfkjf";
      final invalidName2 = "";

      expect(Validate.fullname(validName1), true);
      expect(Validate.fullname(validName2), true);

      // Name cannot be more than 70 characters long
      expect(Validate.fullname(invalidName1), false);

      // Name cannot be empty
      expect(Validate.fullname(invalidName2), false);
    });

    test('Test username validator', () {
      final validUsername1 = "cy6erlion";
      final validUsername2 = "cy6er_lion";
      final validUsername3 = "cy6er1ion2022";
      final validUsername4 = "fishcanyon10";

      final invalidUsername1 = "_cy6erlion";
      final invalidUsername2 = "cy6er.lion";
      final invalidUsername3 = "cy6er__lion";
      final invalidUsername4 =
          "cy6erlion1111111111111111111111111eweewewewewew";
      final invalidUsername5 = "";

      expect(Validate.username(validUsername1), true);
      expect(Validate.username(validUsername2), true);
      expect(Validate.username(validUsername3), true);
      expect(Validate.username(validUsername4), true);

      expect(Validate.username(invalidUsername1), false);
      expect(Validate.username(invalidUsername2), false);
      expect(Validate.username(invalidUsername3), false);
      expect(Validate.username(invalidUsername4), false);
      expect(Validate.username(invalidUsername5), false);
    });

    test('Test password validator', () {
      final validPassword = "thisIs_a_P@ssword";
      final invalidPassword = "short";

      expect(Validate.password(validPassword), true);
      expect(Validate.password(invalidPassword), false);
    });

    test('Test email validator', () {
      final validEmail = "info@example.com";
      final invalidEmail1 = "wrong";
      final invalidEmail2 = "";

      expect(Validate.email(validEmail), true);
      expect(Validate.email(invalidEmail1), false);
      expect(Validate.email(invalidEmail2), false);
    });
  });
}
