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

Requires `write:series` scope in the Account.

### Example request

```http
PUT /v1/accounts/0/series/example-1/000001 HTTP/1.1
Host: localhost:3030
Content-Type: application/json
Authorization: Basic emJXSnJLVXBEWkE4akdqRk4wNGxEQnBkOmZoN1ZoMEVmZ29GUGxPQU1ySk1NVTFFV1dBcGdFRjJscGw5MzAyM3VhMVlyaFd1TEZweE5lRGxYM3Q5aE45UVltTG90V2g1ejM5bk1ocXp6WnN1Z3lDNkc2b2xRNGlLQ3BnNnFVczF2ZFpsRkFjU1FHWnRIU1Y1TnpqNFNxQmZ5

```

### Example response

```http
HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 61
Date: Tue, 03 May 2022 00:14:04 GMT

{
  "creator_id": 1,
  "key": "example-1",
  "token": "000001",
  "value": 1
}
```


## GET /v1/accounts/{account_id}/series/{series_id}

Fetch the state of the specified Series.

Requires `read:series` scope in the Account.

### Example request

```http
GET /v1/accounts/0/series/example-1 HTTP/1.1
Host: localhost:3030
Content-Type: application/json
Authorization: Basic SlpnN1FncHl1Q0RZeFhQN0ozWEZGZHRZOm5YSHByMXdReUVTcWJBN1hpc3Z6WHR3MUl5bTVxY0RCZ2ZydmtrUDg5cHc5YTJuVEd4cUV5cWt3bHVpSkJYU2RCZHlsc3VwYjJGTjk3cWw3MmtFbnF1dFJ5QVNVODVRZW5MdnFuOEZjRXFyUlJEa3JKMVVNdE9wOExKSEN5TG9o

```

### Example response

```http
HTTP/1.1 200 OK
Date: Tue, 03 May 2022 00:14:04 GMT
Content-Type: application/json
Content-Length: 61

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

Requires `read:series` scope in the Account.

### Example request

```http
GET /v1/accounts/0/series/example-1/000001 HTTP/1.1
Host: localhost:3030
Content-Type: application/json
Authorization: Basic SlpnN1FncHl1Q0RZeFhQN0ozWEZGZHRZOm5YSHByMXdReUVTcWJBN1hpc3Z6WHR3MUl5bTVxY0RCZ2ZydmtrUDg5cHc5YTJuVEd4cUV5cWt3bHVpSkJYU2RCZHlsc3VwYjJGTjk3cWw3MmtFbnF1dFJ5QVNVODVRZW5MdnFuOEZjRXFyUlJEa3JKMVVNdE9wOExKSEN5TG9o

```

### Example response

```http
HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 61
Date: Tue, 03 May 2022 00:14:04 GMT

{
  "creator_id": 0,
  "key": "example-1",
  "token": "000001",
  "value": 1
}
```


## DELETE /v1/accounts/{account_id}/series/{series_id}

Delete a Series. This will delete the entire history of the
Series. You may reuse the Series after it has been deleted and
the value will be reset to `1`.

Requires `delete:series` scope in the Account.

### Example request

```http
DELETE /v1/accounts/0/series/example-1 HTTP/1.1
Host: localhost:3030
Content-Type: application/json
Authorization: Basic emJXSnJLVXBEWkE4akdqRk4wNGxEQnBkOmZoN1ZoMEVmZ29GUGxPQU1ySk1NVTFFV1dBcGdFRjJscGw5MzAyM3VhMVlyaFd1TEZweE5lRGxYM3Q5aE45UVltTG90V2g1ejM5bk1ocXp6WnN1Z3lDNkc2b2xRNGlLQ3BnNnFVczF2ZFpsRkFjU1FHWnRIU1Y1TnpqNFNxQmZ5

```

### Example response

```http
HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 0
Date: Tue, 03 May 2022 00:14:04 GMT


```


