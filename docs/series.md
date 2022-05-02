# Monotonic Series

## Contents

* [PUT /v1/accounts/{account_id}/series/{series_id}/{token}](#put-v1accountsaccount_idseriesseries_idtoken)
* [GET /v1/accounts/{account_id}/series/{series_id}](#get-v1accountsaccount_idseriesseries_id)
* [GET /v1/accounts/{account_id}/series/{series_id}/{token}](#get-v1accountsaccount_idseriesseries_idtoken)
* [DELETE /v1/accounts/{account_id}/series/{series_id}](#delete-v1accountsaccount_idseriesseries_id)

## PUT /v1/accounts/{account_id}/series/{series_id}/{token}

Increment a the specified Series for the provided token.

This operation is idempotent for the Series and token. Repeating
an identical request will not repeatedly increment the value of the
Series.

Performing this request again with a different token, or performing
this request with the same token _after_ having performed it with a
different token _will_ increment the series.

Requires `write:entry` scope in the Account.

### Example request

```http
PUT /v1/accounts/0/series/example-1/000001 HTTP/1.1
Host: localhost:3030
Content-Type: application/json
Authorization: Basic eTJNbnQ5bjJQUFZncXJvZlZJRTJNZVFlOlpselJEaGZUYnRTaU1DQ1ZMSUF4SHhTTmd3U2Q1YVp6NkJLTk5RMzc1eGFIWEV5Q0cxN0NBTDc5ZUtlbnJTajU3UTk4elVwR2J5M0R3TVFySGdyZkxUaEthek1kem9QOUdsWXdGcFJiTE1rVG1hMFJoTW9VY1JoV2ZYa3dLUktW

```

### Example response

```http
HTTP/1.1 200 OK
Content-Length: 61
Date: Mon, 02 May 2022 13:23:10 GMT
Content-Type: application/json

{
  "creator_id": 1,
  "key": "example-1",
  "token": "000001",
  "value": 1
}
```


## GET /v1/accounts/{account_id}/series/{series_id}

Fetch the state of the specified Series.

Requires `read:entry` scope in the Account.

### Example request

```http
GET /v1/accounts/0/series/example-1 HTTP/1.1
Host: localhost:3030
Content-Type: application/json
Authorization: Basic cVFsd0RJVkxNWEFpam5ROUdVUHIzT3cxOmtPeTN5dEtuc3dkMzdiN3Y5QnRsZkxyMTQxcmtXbjFWT0tjZWVoWUdHV3FpRXZsYlpqZm1PQnllaFMzOUFxNnpteFlYeWhaNEJHVW1FSmh4YTYyQ1JDV3BDWVdJenhpN1ZGZFZrcjBOVGZKMVlWQmN4YlpHdG5XQXl1ZVAxNUt4

```

### Example response

```http
HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 61
Date: Mon, 02 May 2022 13:23:10 GMT

{
  "creator_id": 0,
  "key": "example-1",
  "token": "000001",
  "value": 1
}
```


## GET /v1/accounts/{account_id}/series/{series_id}/{token}

Fetch the state of the specified Series for a specific token
which has previously been used to increment the Series.

Requires `read:entry` scope in the Account.

### Example request

```http
GET /v1/accounts/0/series/example-1/000001 HTTP/1.1
Host: localhost:3030
Authorization: Basic cVFsd0RJVkxNWEFpam5ROUdVUHIzT3cxOmtPeTN5dEtuc3dkMzdiN3Y5QnRsZkxyMTQxcmtXbjFWT0tjZWVoWUdHV3FpRXZsYlpqZm1PQnllaFMzOUFxNnpteFlYeWhaNEJHVW1FSmh4YTYyQ1JDV3BDWVdJenhpN1ZGZFZrcjBOVGZKMVlWQmN4YlpHdG5XQXl1ZVAxNUt4
Content-Type: application/json

```

### Example response

```http
HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 61
Date: Mon, 02 May 2022 13:23:10 GMT

{
  "creator_id": 0,
  "key": "example-1",
  "token": "000001",
  "value": 1
}
```


## DELETE /v1/accounts/{account_id}/series/{series_id}

Delete a Series. This will delete the entire history of the
series. You may reuse the Series after it has been deleted and
the value will reset to `1`.

Requires `delete:entry` scope in the Account.

### Example request

```http
DELETE /v1/accounts/0/series/example-1 HTTP/1.1
Host: localhost:3030
Content-Type: application/json
Authorization: Basic eTJNbnQ5bjJQUFZncXJvZlZJRTJNZVFlOlpselJEaGZUYnRTaU1DQ1ZMSUF4SHhTTmd3U2Q1YVp6NkJLTk5RMzc1eGFIWEV5Q0cxN0NBTDc5ZUtlbnJTajU3UTk4elVwR2J5M0R3TVFySGdyZkxUaEthek1kem9QOUdsWXdGcFJiTE1rVG1hMFJoTW9VY1JoV2ZYa3dLUktW

```

### Example response

```http
HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 0
Date: Mon, 02 May 2022 13:23:11 GMT


```


