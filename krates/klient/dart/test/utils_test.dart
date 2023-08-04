import 'package:kong/kong.dart';
import 'package:test/test.dart';

void main() {
  group('Tests for kong utilities', () {
    test('Test extractToken function', () {
      final cookieString =
          "bizkit_cookie=dGltQG9tYXRhbGEtbm9kZTIwMjMtMDgtMDQgMTY6NTU6MTEuNzkyMjMzNDU0IFVUQ6scW3DC_sTNY3b0QINsmaqATn1hSh8lCvXw5-cz3AEg; Secure; HttpOnly";

      expect(extractToken(cookieString).length, 108);
      expect(
          extractToken(cookieString)
              .startsWith("dGltQG9tYXRhbGEtbm9kZTIwMjMtMDgtMDQgMT"),
          true);
    });

    test('Test isAlphaNumeric function', () {
      expect(isAlphanumeric("a"), isTrue);
      expect(isAlphanumeric("b"), isTrue);
      expect(isAlphanumeric("c"), isTrue);
      expect(isAlphanumeric("d"), isTrue);
      expect(isAlphanumeric("e"), isTrue);
      expect(isAlphanumeric("f"), isTrue);
      expect(isAlphanumeric("g"), isTrue);
      expect(isAlphanumeric("h"), isTrue);
      expect(isAlphanumeric("i"), isTrue);
      expect(isAlphanumeric("j"), isTrue);
      expect(isAlphanumeric("k"), isTrue);
      expect(isAlphanumeric("l"), isTrue);
      expect(isAlphanumeric("m"), isTrue);
      expect(isAlphanumeric("n"), isTrue);
      expect(isAlphanumeric("o"), isTrue);
      expect(isAlphanumeric("p"), isTrue);
      expect(isAlphanumeric("q"), isTrue);
      expect(isAlphanumeric("r"), isTrue);
      expect(isAlphanumeric("s"), isTrue);
      expect(isAlphanumeric("t"), isTrue);
      expect(isAlphanumeric("u"), isTrue);
      expect(isAlphanumeric("v"), isTrue);
      expect(isAlphanumeric("w"), isTrue);
      expect(isAlphanumeric("x"), isTrue);
      expect(isAlphanumeric("y"), isTrue);
      expect(isAlphanumeric("z"), isTrue);
      expect(isAlphanumeric("A"), isTrue);
      expect(isAlphanumeric("B"), isTrue);
      expect(isAlphanumeric("C"), isTrue);
      expect(isAlphanumeric("D"), isTrue);
      expect(isAlphanumeric("E"), isTrue);
      expect(isAlphanumeric("F"), isTrue);
      expect(isAlphanumeric("G"), isTrue);
      expect(isAlphanumeric("H"), isTrue);
      expect(isAlphanumeric("I"), isTrue);
      expect(isAlphanumeric("J"), isTrue);
      expect(isAlphanumeric("K"), isTrue);
      expect(isAlphanumeric("L"), isTrue);
      expect(isAlphanumeric("M"), isTrue);
      expect(isAlphanumeric("N"), isTrue);
      expect(isAlphanumeric("O"), isTrue);
      expect(isAlphanumeric("P"), isTrue);
      expect(isAlphanumeric("Q"), isTrue);
      expect(isAlphanumeric("R"), isTrue);
      expect(isAlphanumeric("S"), isTrue);
      expect(isAlphanumeric("T"), isTrue);
      expect(isAlphanumeric("U"), isTrue);
      expect(isAlphanumeric("V"), isTrue);
      expect(isAlphanumeric("W"), isTrue);
      expect(isAlphanumeric("X"), isTrue);
      expect(isAlphanumeric("Y"), isTrue);
      expect(isAlphanumeric("Z"), isTrue);
      expect(isAlphanumeric("1"), isTrue);
      expect(isAlphanumeric("2"), isTrue);
      expect(isAlphanumeric("3"), isTrue);
      expect(isAlphanumeric("4"), isTrue);
      expect(isAlphanumeric("5"), isTrue);
      expect(isAlphanumeric("6"), isTrue);
      expect(isAlphanumeric("7"), isTrue);
      expect(isAlphanumeric("8"), isTrue);
      expect(isAlphanumeric("9"), isTrue);
      expect(isAlphanumeric("0"), isTrue);

      expect(isAlphanumeric("!"), isFalse);
      expect(isAlphanumeric("@"), isFalse);
      expect(isAlphanumeric("#"), isFalse);
      expect(isAlphanumeric("\$"), isFalse);
      expect(isAlphanumeric("%"), isFalse);
      expect(isAlphanumeric("^"), isFalse);
      expect(isAlphanumeric("&"), isFalse);
      expect(isAlphanumeric("*"), isFalse);
      expect(isAlphanumeric("("), isFalse);
      expect(isAlphanumeric(")"), isFalse);
      expect(isAlphanumeric("-"), isFalse);
      expect(isAlphanumeric("="), isFalse);
      expect(isAlphanumeric("+"), isFalse);
      expect(isAlphanumeric("\\"), isFalse);
      expect(isAlphanumeric("]"), isFalse);
      expect(isAlphanumeric("}"), isFalse);
      expect(isAlphanumeric("["), isFalse);
      expect(isAlphanumeric("{"), isFalse);
      expect(isAlphanumeric(":"), isFalse);
      expect(isAlphanumeric(";"), isFalse);
      expect(isAlphanumeric("'"), isFalse);
      expect(isAlphanumeric("\""), isFalse);
      expect(isAlphanumeric("<"), isFalse);
      expect(isAlphanumeric(">"), isFalse);
      expect(isAlphanumeric("/"), isFalse);
      expect(isAlphanumeric("?"), isFalse);
      expect(isAlphanumeric("."), isFalse);
      expect(isAlphanumeric(","), isFalse);
    });
  });
}
