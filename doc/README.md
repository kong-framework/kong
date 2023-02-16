---
title: 🕸️🔐 Kong
subtitle: Secure Web Node
author: Jackson G. Kaindume <kaindume@proton.me>
date: 2022
...
---

# 🕸️🔐

Kong is a secure web node.

See also:

- [🌐 Kong API Documentation](./api.html)
- [🗄️ Kong Data Structures Documentation](./data.html)

## ✅ Features

- Confidentiality
- Intergrity
- Availability
- Database System
- Session Management
- HTTP+JSON API
- Optional Federation
- Built in HTTP server
- Account Management

___

## 🧩 Components

- [`web`](https://codeberg.org/kong/web): simple, secure mechanical web server framework [`rust`]
- [`node`](https://codeberg.org/kong/node): secure web node [`rust`]
- [`spinneret`](https://codeberg.org/kong/spinneret): Container runtime [`rust`]
- [`h4`](https://codeberg.org/kong/h4): h4cking 700l$ [`rust`]
- [`client`](https://codeberg.org/kong/client): Kong HTTP client. [`dart`]

___

## 🗃️ Database

Kong stores structured data in a database. The following data is
is stored:

1. Accounts

### Sqlite Support

Sqlite is supported in Kong as an optional feature:

```toml
Kong = { path = "../../kong/phora", features = ["sqlite"] }
```
When the sqlite feature is enabled user account data will be saved in
a sqlite database.

___

## 🖥️ Node

### Cargo Features

| Feature            | Description                                                                                          |
|--------------------|------------------------------------------------------------------------------------------------------|
| `sqlite`           | Use sqlite as the Node's database system                                                             |
| `postgres`         | Use postgres as the Node's database system                                                           |
| `node_information` | Enables an API route that serves information about the Kong node                               |

### Usage

#### 1. Cargo.toml

Import the `Kong` crate in your project, with the required 
features:

```toml
Kong = { git = "https://codeberg.org/kong/node.git" }
```

#### 2. Use crate

```rust

```
___

## 🕸️ Web

> simple mechanical web server framework

### Goals

- Browsers should not should'nt need to guess the __content type__ of
a document, it is up to the server to tell the browser (With the 
`Content-type` header).

___

### Usage

```rust
main {
  let routes = vec![router::Route {
    path: "/".to_string(),
    handler: route_handler,
  }];
  let router = Router::new(routes);
	
  let web = Web {
    address: "127.0.0.1:7878".to_string(),
    router,
  };

  // Start http server listening for the routes
  web.start();
}

// Route handler
fn route_handler<'a>(_con: TcpStream, _req: Request) {
  unimplemented!();
}
```
___

## 🔐 Security
### Password Hashing

#### Why hash?

It is only a matter of time until your server gets hacked, and
when that happens you don't want the users passwords to be leaked --
this will allow the attacker to gain access to the users resources.
Some users also use the same password across many services, your 
web-server can be the root cause of a chain of breaches.

A cool way to prevent this type of leak is by __obfuscating__ the 
users password with a [__hash function__](https://en.wikipedia.org/wiki/Hash_function).

There are lots of hash functions that can be used, but most of these 
will be a bad idea to use. For example if you use SHA-256 or other 
computationally cheap functions (hash function without a __work factor__ 
parameter), they are vulnerable to rainbow table attacks.
Bruteforce is also possible if the password length is short/known, 
asic miners can generate 100 TeraHashes PER Second.

The server can increase the passwords entropy by concatenating it with
a random string aka the __salt__. Users can also protect themselves 
by using longer passwords.

The best method to use against plaintext password leaks and rainbow
table attacks is to use a __Password Hash Function__. Which is a hash 
function specially designed to be slow/expensive to compute even on
specialized hardware.

#### Scrypt [recommended]

The [scrypt](https://www.tarsnap.com/scrypt.html) hash function uses large amounts of memory when hashing 
making it expensive to scale to the point of reasonable bruteforce 
attacks. Secure against hardware brute-force attacks.

A number of cryptocurrencies use __scrypt__ for proof of work.

Created by Colin Percival of [Tarsnap](https://en.wikipedia.org/wiki/Tarsnap)

#### Argon2d [recommended]

The [Argon2d](https://en.wikipedia.org/wiki/Argon2) function is 
designed to resist GPU cracking attacks. Secure against hardware 
brute-force attacks.

It is the winner of [Password Hashing Competition](https://www.password-hashing.net/).

#### Bcrypt

[Bcrypt](https://en.wikipedia.org/wiki/Bcrypt) is based on the 
[blowfish](https://en.wikipedia.org/wiki/Blowfish_(cipher)) cipher.

Vulnerable against hardware brute-force attacks.

#### PBKDF2

[PBKDF2](https://en.wikipedia.org/wiki/PBKDF2) is an key derivation
function with a sliding computational cost to reduce bruteforce 
search.

Vulnerable against hardware brute-force attacks.

#### Conclusion

A cool way to prevent password leaks is by __obfuscating__ them
with a password hash functions which offer additional security 
against bruteforce from specialliazed hardware such as asics. If 
password hash functions are used and implemented correctly even the 
administrators of the server will not be able to read the users 
passwords especially if the server is open source and the users can
audit the code for themselves.

<https://www.troyhunt.com/our-password-hashing-has-no-clothes/>
<https://paragonie.com/blog/2016/02/how-safely-store-password-in-2016>
<https://www.troyhunt.com/passwords-evolved-authentication-guidance-for-the-modern-era/>

___

## Validation

<https://beesbuzz.biz/code/439-Falsehoods-programmers-believe-about-email>

___
