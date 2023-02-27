---
title: ⬡ Kong
subtitle: Secure Web Node
author: Jackson G. Kaindume <cy6erlion@protonmail.com>
date: 2020-2023
...
---


``` text
                              )                 
                           ( /(          (  (   
                            )\())(   (    )\))(  
                           ((_)\ )\  )\ )((_))\  
                           | |(_|(_)_(_/( (()(_) 
                           | / / _ \ ' \)) _` |  
                           |_\_\___/_||_|\__, |  
                         secure web node |___/ v0.1.0
```

# ⬡ Kong

See also:

- [Kong API Documentation](./api.html)
- [Kong Data Structures Documentation](./data.html)

## ✅ Features

- [ ] Confidentiality
- [ ] Integrity
- [ ] Availability
- [ ] Database Management
- [ ] Session Management
- [ ] HTTP+JSON API
- [ ] Optional Federation
- [ ] Built in HTTP server
- [ ] Account Management

___

## 🧩 `krates`

Kong consists of severel components known as `krates`.

- [[kdata]](data.html): All the common data types used in kong
- [[kerror]](#kerror): Error types
- [[klient]](#klient): Clients in Javascript and Dart
- [[kollection]](#kollection): database management
- [[konfig]](#konfig): Node configuration
- [[kong]](#kong): Core server runtime
- [[konja]](#konja): Pen-testing tools
- [[kontainer]](#kontainer): LXC container management
- [[krypto]](#krypto): Cryptography and security related code

___

## kollection

Kong stores structured data in a database. [SQLite](https://sqlite.org/)
is the database management system that is used, with help from the 
[rusqlite](https://sqlite.org/) crate.

The following data is stored the database:

1. [`Accounts`](./data.html#account)

---

## krypto

### Password Hashing

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

### Authentication and Authorization


