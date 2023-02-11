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

___

## ℹ️ `NodeInfo`

| Field     | Description                                   | Type     |
|-----------|-----------------------------------------------|----------|
| `version` | Version of `kong`  running on the node. | `String` |
