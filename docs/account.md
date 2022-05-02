# Account Management

## Contents

* [GET /v1/accounts/{account_id}](#get-v1accountsaccount_id)

## GET /v1/accounts/{account_id}

Fetch an Account.

Requires `read:account` scope in the Account.

### Example request

```http
GET /v1/accounts/0 HTTP/1.1
Host: localhost:3030
Authorization: Basic dGVzdGFwaTpzZWNyZXQxMjM=

```

### Example response

```http
HTTP/1.1 200 OK
Content-Length: 8
Date: Mon, 02 May 2022 13:18:58 GMT
Content-Type: application/json

{
  "id": 0
}
```


