---
title: 🌐 Kong API
subtitle: Secure Web Node
author: Jackson G. Kaindume <kaindume@proton.me>
date: 2022
...
---

# 🌐

Documentation of the Kong HTTP APIs.

___

## ℹ️ `/node-information`

Gets information about the running node.

| Overview                              |
|---------------------------------------|
| Resource: __`/node-information`__     |
| Architype: __`Document`__             |
| Cargo feature: __`node_information`__ |

__Methods:__

### HEAD `/node-information`

### GET `/node-information`

- __Success Response Codes__: 
  - {__OK 200__, [`NodeInfo`]()}
- __Error Response Codes__: 
  - {__500__, `TODO`}

#### HTTP HEADERS & Media Types

// TODO
___

## 👤 `/accounts`

Management of user accounts.

| Overview                    |
|-----------------------------|
| Resource: __`/accounts`__   |
| Architype: __`Collection`__ |
| Cargo feature: `None`       |

__Methods:__

### POST `/accounts`

Creates a new account object.

- Input: `AccountCreationData`
- __Success Response Codes__: 
  - {__OK 201__, `TODO`}
- __Error Response Codes__: 
  - {__400__, `TODO`}
  - {__500__, `TODO`}

#### HTTP HEADERS & Media Types

// TODO
