---
title: 🗄️ Kong Data
subtitle: Secure Web Node
author: Jackson G. Kaindume <kaindume@proton.me>
date: 2022
...
---

# 🗄️

Documentation of the data structures used in `kong`.

## 👤 `Account`

A user's account.

| Field         | Description                       | Type                                       |
|---------------|-----------------------------------|--------------------------------------------|
| `username`    | Account's unique username.        | `String`                                   |
| `password`    | Account's master key              | `String`                                   |
| `personal`    | Account's personal information    | [`Option<PersonalData>`](#personal-data)   |
| `education`   | Account's educational information | [`Option<EducationData>`](#education-data) |
| `bussiness`   | Account's bussiness information   | [`Option<BussinessData>`](#bussiness-data) |
| `contact`     | Account's contact information     | [`Option<ContactData>`](#contact-data)     |
| `description` | Short bio of Account              | `Option<String>`                           |
| `last_login`  | Date user last logged in          | `Option<Date>`                             |
| `created`     | Date when account was created     | `Date`                                     |


#### `username` format

- cannot start with a underscore (_)
- can only contain letters, numbers, and one underscore
- can have only one (_) underscore


#### `password` format

- should be at least 10 characters long

### ✨ AccountCreationData

Data used as input to create a new account.

| Field              | Description               | Type     |
|--------------------|---------------------------|----------|
| `username`         | Account's unique username | `String` |
| `password`         | Account's master key      | `String` |
| `retyped_password` | Retyped password          | `String` |
| `email`            | User's email address      | `String` |

### 💵 `BussinessData`

Account's bussiness information. This is an optional feature that is enabled
with the `acc_bussiness` cargo feature flag for the [kong node](#node).

| Field            | Description                             | Type     |
|------------------|-----------------------------------------|----------|
| `bussiness_name` | Name of bussiness  who owns the account | `String` |

### ☎️ `ContactData`

Account owner's contact information. This is an optional feature that 
is enabled with the `acc_contact` cargo feature flag for the 
kong node.

| Field           | Description                   | Type             |
|-----------------|-------------------------------|------------------|
| `email`         | Account owner's email address | `Option<String>` |
| `mobile_number` | Account owner's mobile number | `Option<String>` |
| `website`       | Account owner's web-address   | `Option<String>` |

### 🎓 `EducationData`

Account owner's education information. This is an optional feature that 
is enabled with the `acc_education` cargo feature flag for the 
kong node.

| Field            | Description                | Type             |
|------------------|----------------------------|------------------|
| `current_school` | User's current school name | `Option<String>` |
| `student_number` | User's student number      | `Option<String>` |

### 😎 `PersonalData`

Account owner's personal information. This is an optional feature that 
is enabled with the `acc_personal` cargo feature flag for the 
kong node.

| Field           | Description                                | Type             |
|-----------------|--------------------------------------------|------------------|
| `fullname`      | The fullname of the account holder.        | `Option<String>` |
| `date_of_birth` | The date when the account holder was born. | `Option<Data>`   |
| `id_number`     | National ID number of the account owner    | `Option<String>` |
| `gender`        | The gender of the account owner.           | `Option<String>` |

___

## ℹ️ `NodeInfo`

| Field     | Description                                   | Type     |
|-----------|-----------------------------------------------|----------|
| `version` | Version of `kong`  running on the node. | `String` |
