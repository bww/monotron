# Access Control Management

## Contents

* [POST /v1/accounts/{account_id}/grants](#post-v1accountsaccount_idgrants)
* [GET /v1/accounts/{account_id}/grants](#get-v1accountsaccount_idgrants)
* [GET /v1/accounts/{account_id}/grants/{api_key}](#get-v1accountsaccount_idgrantsapi_key)
* [DELETE /v1/accounts/{account_id}/grants/{api_key}](#delete-v1accountsaccount_idgrantsapi_key)

## POST /v1/accounts/{account_id}/grants

Create an API Key in the specified Account.

Requires `write:acl` scope in the Account.

### Example request

```http
POST /v1/accounts/0/grants HTTP/1.1
Host: localhost:3030
Content-Type: application/json
Authorization: Basic dGVzdGFwaTpzZWNyZXQxMjM=
Content-Length: 25

[
  "read,write:entry"
]
```

### Example response

```http
HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 238
Date: Mon, 02 May 2022 13:18:58 GMT

{
  "account_id": 0,
  "api_key": {
    "id": 2,
    "key": "RwaeAGyByDYXWTgmr0hy03eK",
    "secret": "vtS3RtbCRVSGxpje1xskTweo2Q3mW4HukrJRb1sqKBUnVIMzD5oCuC6LLf1KLoH00ep6DKgK8otWMsT4lWWagahZ97wHl4t7eRrXcGSHAr5dJfqSkJYGe7RMTznAwGMB"
  },
  "scopes": [
    "read,write:entry"
  ]
}
```


## GET /v1/accounts/{account_id}/grants

List API Keys in the specified Account.

Requires `read:acl` scope in the Account.

### Example request

```http
GET /v1/accounts/0/grants HTTP/1.1
Host: localhost:3030
Content-Type: application/json
Authorization: Basic WU4xYlJTdUR0QjNvNDhkWXdKSTdYN0lwOnZOUnBKTHpQN3RFRFFaRjVwRjloSDdJOGNqeFhnNUZmamVBTlJhVkt4cDlTZ3Vvd2JQU0lEQlFMMW9uTGEwa2Z0ZE50WVlYc2dBM1E5T0ZwV3F4ckhJTWlUZ3loNldkanlQMVBMdGN6N2lpVDlNYmt1enlFQnp6TFFjbGR2TTFa

```

### Example response

```http
HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 705
Date: Mon, 02 May 2022 13:18:58 GMT

[
  {
    "account_id": 0,
    "api_key": {
      "id": 2,
      "key": "RwaeAGyByDYXWTgmr0hy03eK",
      "secret": "vtS3RtbCRVSGxpje1xskTweo2Q3mW4HukrJRb1sqKBUnVIMzD5oCuC6LLf1KLoH00ep6DKgK8otWMsT4lWWagahZ97wHl4t7eRrXcGSHAr5dJfqSkJYGe7RMTznAwGMB"
    },
    "scopes": [
      "read,write:entry"
    ]
  },
  {
    "account_id": 0,
    "api_key": {
      "id": 3,
      "key": "YN1bRSuDtB3o48dYwJI7X7Ip",
      "secret": "vNRpJLzP7tEDQZF5pF9hH7I8cjxXg5FfjeANRaVKxp9SguowbPSIDBQL1onLa0kftdNtYYXsgA3Q9OFpWqxrHIMiTgyh6WdjyP1PLtcz7iiT9MbkuzyEBzzLQcldvM1Z"
    },
    "scopes": [
      "read,write:acl"
    ]
  },
  {
    "account_id": 0,
    "api_key": {
      "id": 4,
      "key": "mLpdyBPhssd2CPM6Zotf9KM6",
      "secret": "3AE9Ce26OOaWR1UZ4t5xCmpWjWnd25vD5rG3PS2hE8B0zd7SAoDnO1JCoOTeaMEELbLwWwzUghmNSuovdpj6gI661eiZ2CQP4fcogGnMMfhWuPz61C6SMtmCp5E8eMIP"
    },
    "scopes": [
      "*:acl"
    ]
  }
]
```


## GET /v1/accounts/{account_id}/grants/{api_key}

Fetch an API Key in the specified Account.

Requires `read:acl` scope in the Account.

### Example request

```http
GET /v1/accounts/0/grants/RwaeAGyByDYXWTgmr0hy03eK HTTP/1.1
Host: localhost:3030
Content-Type: application/json
Authorization: Basic WU4xYlJTdUR0QjNvNDhkWXdKSTdYN0lwOnZOUnBKTHpQN3RFRFFaRjVwRjloSDdJOGNqeFhnNUZmamVBTlJhVkt4cDlTZ3Vvd2JQU0lEQlFMMW9uTGEwa2Z0ZE50WVlYc2dBM1E5T0ZwV3F4ckhJTWlUZ3loNldkanlQMVBMdGN6N2lpVDlNYmt1enlFQnp6TFFjbGR2TTFa

```

### Example response

```http
HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 238
Date: Mon, 02 May 2022 13:18:58 GMT

{
  "account_id": 0,
  "api_key": {
    "id": 2,
    "key": "RwaeAGyByDYXWTgmr0hy03eK",
    "secret": "vtS3RtbCRVSGxpje1xskTweo2Q3mW4HukrJRb1sqKBUnVIMzD5oCuC6LLf1KLoH00ep6DKgK8otWMsT4lWWagahZ97wHl4t7eRrXcGSHAr5dJfqSkJYGe7RMTznAwGMB"
  },
  "scopes": [
    "read,write:entry"
  ]
}
```


## DELETE /v1/accounts/{account_id}/grants/{api_key}

Delete an API Key in the specified Account.

Requires `delete:acl` scope in the Account.

### Example request

```http
DELETE /v1/accounts/0/grants/RwaeAGyByDYXWTgmr0hy03eK HTTP/1.1
Host: localhost:3030
Content-Type: application/json
Authorization: Basic bUxwZHlCUGhzc2QyQ1BNNlpvdGY5S002OjNBRTlDZTI2T09hV1IxVVo0dDV4Q21wV2pXbmQyNXZENXJHM1BTMmhFOEIwemQ3U0FvRG5PMUpDb09UZWFNRUVMYkx3V3d6VWdobU5TdW92ZHBqNmdJNjYxZWlaMkNRUDRmY29nR25NTWZoV3VQejYxQzZTTXRtQ3A1RThlTUlQ

```

### Example response

```http
HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 0
Date: Mon, 02 May 2022 13:18:58 GMT


```


