---
title: 🔐 Kong Cryptography and Security
author: Jackson G. Kaindume <cy6erlion@protonmail.com>
date: Last update 28.02.2023
...
---

# 🔐

Documentation of the cryptography and cyber-security measures
used in `kong`.

---

## 🔑 Authentication

- [ ] __Username__ and __password__ is used to authenticate users (humans). 
- [x] Usernames are __alphanumeric__ (letters A-Z, numbers 0-9) with the exception of __underscores__.
- [x] Password should be at least 10 characters long
- [x] The user's password is [hashed](file:///home/kaindume/kwatafana/kong/doc/pub/krypto.html#password-hashing)
with `scrypt` and the hash is stored in the database.
- [ ] The username may be claimed by a suspended or deactivated 
account. Suspended and deactivated usernames are not immediately 
available for use.
- [ ] After the user has been authenticated, they are handed a 
__passport__ that should send with requests to private resources. 
 

TODO:

> The `kong` allows you to reserve a list of usernames that cannot 
> never used by end-users (e.g __admin__),

---

## 🔏  Password Hashing
#### Why hash?

Web servers can get hacked, and when that happens you don't want the 
passwords of the users to be leaked -- this will allow the attacker 
to gain access to the users resources. Some users also use the same 
password across many services, your web-server can be the root of a 
chain of breaches.

A cool way to prevent this type of leak is by __obfuscating__ the 
users passwords with a [__hash function__](https://en.wikipedia.org/wiki/Hash_function),
before storing them in the database.

There are lots of hash functions that can be used, but most of these 
are a bad choice. For example if you choose SHA-256 or other 
computationally cheap functions (hash function without a __work factor__ 
parameter), they will be vulnerable to rainbow table attacks.
Bruteforce is also easy if the password length is short/known, 
asic miners can be used for bruteforce search, they can generate 100 
TeraHashes PER Second.

#### Password Hash Functions

The best method to use against plaintext password leaks and rainbow
table attacks is to use a __Password Hash Function__. Which is a hash 
function specially designed to be slow/expensive to compute/bruteforce
even on specialized hardware.

#### Scrypt Hash Function

The [scrypt](https://www.tarsnap.com/scrypt.html) hash function uses large amounts of memory when hashing, 
making it expensive to scale bruteforce attacks. `scrypt` is also 
secure against hardware brute-force attacks.

In `kong` we use [`scrypt`](https://github.com/RustCrypto/password-hashes/tree/master/scrypt)
to hash users passwords before they are stored in the database (`kollection`).

> A number of cryptocurrencies use __scrypt__ for proof of work.

> `scrypt` is created by Colin Percival of [Tarsnap](https://en.wikipedia.org/wiki/Tarsnap)

#### Conclusion

A cool way to prevent password leaks is by __obfuscating__ them
with a password hash function which offer additional security 
against bruteforce from specialized hardware such as asics. If 
password hash functions are used and implemented correctly even the 
administrators of the server will not be able to read the users 
passwords especially if the server is open source and the users can
audit the code for themselves.

#### References

- <https://www.troyhunt.com/our-password-hashing-has-no-clothes/>
- <https://paragonie.com/blog/2016/02/how-safely-store-password-in-2016>
- <https://www.troyhunt.com/passwords-evolved-authentication-guidance-for-the-modern-era/>

___

## 🪪 Kong Passport

An authorization token that uses the `keyed_hash()` function from
__Blake3__ instead of using __HMAC__. It provides authentication
using a secret key. We call such a token `kpassport` (kong passport).

### Format

```text
Base64([HOST][USERNAME][TIMESTAMP][SIGNATURE])
         45B    15B       30B        32B
```

- __HOST__: The issuer of the `kpassport` can be a, the maximum length 
45bytes because that is the maximum IPv6 string length.  But any 
string identifier can be used not just IP addresses as long as it 
fits into 45bytes 
- __USERNAME__: The username of the entity the `kpassport` issued to.
The maximum length is 15bytes because `kong` account username have a 
maximum length of 15 characters.
- __TIMESTAMP__: The time the `kpassport` was issued, 

#### Why use blake3

- Fast
- Pure __Rust__ implementation written by the creators of blake3 
(`kong` is also written in Rust).

#### HTTPS

The HTTP protocol transfers data in cleartext, and a 
"man-in-the-middle" can see all the data (HTTP requests and responses)
being transferred between client and server. This is why an HTTPS 
connection should be used, it creates a secure channel between client 
and server that is not vulnerable to "MITM" attacks.

#### Size

Web browsers limit the amount of storage size a domain can use for 
cookies. All the cookies under one domain cannot exceed 4KB (4093 
bytes per domain). This means you can have 1 cookie of 4093 bytes, 
or 2 cookies of 2045 bytes, etc.

#### Security
- [ ] A `kpassport` is unique
- [ ] A `kpassport` is not guessable (they are randomly generated).

---

## 🚪 Authorization

The main idea is to store the user’s info in the `kpassport`.  And 
to secure it, have the user's info be signed using a secret that’s 
only known to the server.

#### Attaching to HTTP requests
Clients that request to access protected routes, need to provide a
valid `kpassport`, they do this by attaching a `kpassport` with every
request to a protected route. There are two ways to attach a 
`kpassport` to a request:

1. HTTP Cookie

The cookies __Secure__ attribute is set, this ensures that the
cookie is only sent over an HTTPS connection and not HTTP. This means
that the cookie (`kpassport`) cannot be accessed by "MITM" attackers.

The cookies __HttpOnly__ attribute is also set, this ensures that
the cookie is inaccessible to the JavaScript `Document.cookie` API. So
the cookie cannot be read or modified by client side JavaScript.

Cookie expiration date is also set. It is calculated from 
the `kpassport`'s timestamp:

```
TODO: how to calculate a cookies expiration date from a kpassport
```

```
Set-Cookie: session=<kpassport token>; Expires=Thu, 21 Oct 2021 07:28:00 GMT; Secure; HttpOnly
```

2.  Authorization header:

A `kpassport` can be transported using HTTP headers, the 
token is sent in the Authorization header:

```text
Authorization: Bearer <the kpassport token>
```

#### Expiration
A `kpassport` is timestamped at the time it is issued

___

## TODO

- 🛡️ Physical Security
  - Machine Identity
  - Secure Boot Stack 
- 🚀 Secure Service Deployment
- 🎫 Service Identity, Integrity, and Isolation
- 🎭 Inter-Service Access Management
- 🎭 🔐 Encryption of Inter-Service Communication
- 🚪 Access Management of End User Data
- 🚨  Intrusion Detection
- 🎡 Denial of Service (DoS) Protection
- 🔐📄 Secure Data Storage
  - Encryption at Rest
  - Deletion of data

---

[[⬡]](./index.html) | [[⧉]](https://kwatafana.codeberg.page/)
