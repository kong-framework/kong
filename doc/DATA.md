---
title: 🗄️ Kong Data Structures Documentation
subtitle: Secure Web Node
author: Jackson G. Kaindume <kaindume@proton.me>
...
---

# 🗄️

Documentation of the data structures used in `kong`.

## 👤 `Account`

A user's account.

| Field            | Description                                | Type             |
|------------------|--------------------------------------------|------------------|
| `username`       | Account's unique username.                 | `String`         |
| `password`       | Account's master key                       | `String`         |
| `bussiness_name` | Name of bussiness  who owns the account    | `Option<String>` |
| `email`          | Account owner's email address              | `Option<String>` |
| `mobile_number`  | Account owner's mobile number              | `Option<String>` |
| `website`        | Account owner's web-address                | `Option<String>` |
| `current_school` | User's current school name                 | `Option<String>` |
| `student_number` | User's student number                      | `Option<String>` |
| `fullname`       | The fullname of the account holder.        | `Option<String>` |
| `date_of_birth`  | The date when the account holder was born. | `Option<Date>`   |
| `id_number`      | National ID number of the account owner    | `Option<String>` |
| `gender`         | The gender of the account owner.           | `Option<String>` |
| `description`    | Short bio of Account                       | `Option<String>` |
| `last_login`     | Date user last logged in                   | `Option<Date>`   |
| `created`        | Date when account was created              | `Date`           |


#### `username` format

- cannot start with a underscore (_)
- can only contain letters, numbers, and one underscore
- can have only one (_) underscore


#### `password` format

- should be at least 10 characters long

### 🌐 PublicAccount

Account data that can be shown to the public (Other users, guests).

| Field              | Description               | Type     |
|--------------------|---------------------------|----------|
| `username`         | Account's unique username | `String` |

---

## 🏠 `Property`

Data type to represent a physical property such as a house.

| Field           | Description                                            | Type          |
|-----------------|--------------------------------------------------------|---------------|
| `name`          | Name of the property                                   | `String`      |
| `price`         | Price of the property                                  | `Option<f64>` |
| `bedrooms`      | Number of bedrooms                                     | `u16`         |
| `bathrooms`     | Number of bathrooms                                    | `u16`         |
| `sqft`          | Area size                                              | `f64`         |
| `address`       | Address of property                                    | `String`      |
| `agentid`       | ID of Agent representing the property                  | `Option<i64>` |
| `description`   | Description of the property                            | `String`      |
| `online_views`  | Number of online views the property got                | `u64`         |
| `physical_view` | Number of physical irl views the property got          | `u16`         |
| `likes`         | Number of likes for the property                       | `u64`         |
| `bookmarks`     | Number of bookmarks for the property                   | `u64`         |
| `photos`        | Contains a JSON string with paths to the actual images | `String`      |
| `added`         | Date when the property was added                       | `Date`        |

---

## 🕹️ Inputs

Data that is received as input from users, usually other data types are
created from this input.

### ✨ AccountCreationInput

Data used as input to create a new account.

| Field      | Description               | Type             |
|------------|---------------------------|------------------|
| `username` | Account's unique username | `String`         |
| `email`    | User's email address      | `Option<String>` |
| `password` | Account's master key      | `String`         |

### 🔑 AccountLoginInput

Input provided by the user, to log into their account.

| Field      | Description           | Type     |
|------------|-----------------------|----------|
| `username` | Account email address | `String` |
| `password` | Account master key    | `String` |

### ✨ `PropertyInput`

Input provided by the user, to create a new property.

| Field           | Description                                            | Type          |
|-----------------|--------------------------------------------------------|---------------|
| `name`          | Name of the property                                   | `String`      |
| `price`         | Price of the property                                  | `Option<f64>` |
| `bedrooms`      | Number of bedrooms                                     | `u16`         |
| `bathrooms`     | Number of bathrooms                                    | `u16`         |
| `sqft`          | Area size                                              | `f64`         |
| `address`       | Address of property                                    | `String`      |
| `agentid`       | ID of Agent representing the property                  | `Option<i64>` |
| `description`   | Description of the property                            | `String`      |

___

## ℹ️ `NodeInfo`

| Field     | Description                                   | Type     |
|-----------|-----------------------------------------------|----------|
| `version` | Version of `kong`  running on the node. | `String` |
