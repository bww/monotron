# Monotonic Series

## Contents

* [PUT /v1/accounts/{account_id}/series/{series_id}/{token}](#put-v1accountsaccount_idseriesseries_idtoken)
* [GET /v1/accounts/{account_id}/series/{series_id}](#get-v1accountsaccount_idseriesseries_id)
* [GET /v1/accounts/{account_id}/series/{series_id}/{token}](#get-v1accountsaccount_idseriesseries_idtoken)
* [GET /v1/accounts/{account_id}/series/{series_id}/{token}](#get-v1accountsaccount_idseriesseries_idtoken-1)

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
Authorization: Basic WEE0S041eFhqeFd1VmZjazNGVnFiTVhFOlk5VG9hOHlidXdDQkRWZDNrS0dzcUNZeXFCa2I0YUZpTVd3VXVVazllQUp6bmtSTE1ZeFh1VmZ6Z0NPcGxNUEpCQWEybEZNcnlxalRzQWRIbEN5MktUTTU5dklGNGxqWk51elBKOWwySGRrZUlaMU9wZENMckg1YUQ1S3dJcG1L

```

### Example response

```http
HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 61
Date: Mon, 02 May 2022 13:18:58 GMT

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
Authorization: Basic a0diUkdyR0xuZ1ByV3IyR0hwRDlHajlQOm9YSW5HdFNqTXJTclU5NHpxdE40WFFDOHdWY2N3cWJVYkM0WWFIVEZPOGlMZVFFQnhzaVJYYUdwOGhPR3VqbU1qOWN3Vjh2ZU94c1NPcDdnSXVCWWJrdVpwRnZkaDN1bzk5MHFUTmdVRzRGdVNJU1h0Tm13eVU5QVdtM2NPaGJx

```

### Example response

```http
HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 61
Date: Mon, 02 May 2022 13:18:58 GMT

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
Content-Type: application/json
Authorization: Basic a0diUkdyR0xuZ1ByV3IyR0hwRDlHajlQOm9YSW5HdFNqTXJTclU5NHpxdE40WFFDOHdWY2N3cWJVYkM0WWFIVEZPOGlMZVFFQnhzaVJYYUdwOGhPR3VqbU1qOWN3Vjh2ZU94c1NPcDdnSXVCWWJrdVpwRnZkaDN1bzk5MHFUTmdVRzRGdVNJU1h0Tm13eVU5QVdtM2NPaGJx

```

### Example response

```http
HTTP/1.1 200 OK
Date: Mon, 02 May 2022 13:18:58 GMT
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

Delete a Series. This will delete the entire history of the
series. You may reuse the Series after it has been deleted and
the value will reset to `1`.

Requires `delete:entry` scope in the Account.

### Example request

```http
DELETE /v1/accounts/0/series/example-1 HTTP/1.1
Host: localhost:3030
Content-Type: application/json
Authorization: Basic WEE0S041eFhqeFd1VmZjazNGVnFiTVhFOlk5VG9hOHlidXdDQkRWZDNrS0dzcUNZeXFCa2I0YUZpTVd3VXVVazllQUp6bmtSTE1ZeFh1VmZ6Z0NPcGxNUEpCQWEybEZNcnlxalRzQWRIbEN5MktUTTU5dklGNGxqWk51elBKOWwySGRrZUlaMU9wZENMckg1YUQ1S3dJcG1L

```

### Example response

```http
HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 0
Date: Mon, 02 May 2022 13:18:58 GMT


```


