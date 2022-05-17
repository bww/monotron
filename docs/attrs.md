# Monotonic Series Attributes

## Contents

* [PUT /v1/accounts/{account_id}/series/{series_key}/{token}/attrs](#put-v1accountsaccount_idseriesseries_keytokenattrs)
* [GET /v1/accounts/{account_id}/series/{series_key}/{token}/attrs](#get-v1accountsaccount_idseriesseries_keytokenattrs)
* [GET /v1/accounts/{account_id}/series/{series_key}/{token}/attrs/{attribute_key}](#get-v1accountsaccount_idseriesseries_keytokenattrsattribute_key)
* [DELETE /v1/accounts/{account_id}/series/{series_key}/{token}/attrs/{attribute_key}](#delete-v1accountsaccount_idseriesseries_keytokenattrsattribute_key)
* [DELETE /v1/accounts/{account_id}/series/{series_key}/{token}/attrs](#delete-v1accountsaccount_idseriesseries_keytokenattrs)

## PUT /v1/accounts/{account_id}/series/{series_key}/{token}/attrs

Store multiple attributes for a Series at the specified token.

This operation might be more accurately described as a `PATCH`; keys present
in the provied set are stored, but any existing keys not present in the
provided set are not deleted.

Requires `write:series` scope in the Account.

### Example request

```http
PUT /v1/accounts/0/series/example-1/000001/attrs HTTP/1.1
Host: localhost:3030
Content-Length: 52
Content-Type: application/json
Authorization: Basic NkZXa3pNeWJhdUZ3bDlrdk1kN3licEFBOlVFbUpPZkhWWjFJNmpOd2owZlV6SFZBUnpRU0FHV0VPWWZHQktTazNhQTQ4eDkwbDJFWUpuN2VJejU2ek1ORTRwRXh5TkV4WWo1NWQwcjcxb1NJanBDaTFlVlY1TFVQWW9TZGNDSklXSzNGMzd5eGJCbTJEZnNxeHNLSWd5RnND

{
  "first": "1",
  "second": "2",
  "third": "3"
}
```

### Example response

```http
HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 38
Date: Tue, 17 May 2022 22:38:16 GMT

{
  "first": "1",
  "second": "2",
  "third": "3"
}
```


## GET /v1/accounts/{account_id}/series/{series_key}/{token}/attrs

Fetch attributes for a Series at the specified token.

Requires `read:series` scope in the Account.

### Example request

```http
GET /v1/accounts/0/series/example-1/000001/attrs HTTP/1.1
Host: localhost:3030
Authorization: Basic WkhQaHpvZEl1WllpMGJrT1Y5dEdGQWd3OlNaa3BqdlBWVzhOMTNZOU1telJUTkFKdjBJRTJ6bXQ5RENmTWlLZHBoaUpRZmdDSGhGZzVXaUlUYUZuZTVRYnVFSGRkemNXRFY5VWlpQ1RnV1B3Z2E2SXZuRUNCQWppNDNEM1VtVUhtTFFEUzNGdVB6MUtDUkpGZG1TQXZTeFli

```

### Example response

```http
HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 38
Date: Tue, 17 May 2022 22:38:16 GMT

{
  "first": "1",
  "second": "2",
  "third": "3"
}
```


## GET /v1/accounts/{account_id}/series/{series_key}/{token}/attrs/{attribute_key}

Fetch a specific attribute for a Series at the specified token.

Requires `read:series` scope in the Account.

### Example request

```http
GET /v1/accounts/0/series/example-1/000001/attrs/first HTTP/1.1
Host: localhost:3030
Authorization: Basic WkhQaHpvZEl1WllpMGJrT1Y5dEdGQWd3OlNaa3BqdlBWVzhOMTNZOU1telJUTkFKdjBJRTJ6bXQ5RENmTWlLZHBoaUpRZmdDSGhGZzVXaUlUYUZuZTVRYnVFSGRkemNXRFY5VWlpQ1RnV1B3Z2E2SXZuRUNCQWppNDNEM1VtVUhtTFFEUzNGdVB6MUtDUkpGZG1TQXZTeFli

```

### Example response

```http
HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 1
Date: Tue, 17 May 2022 22:38:16 GMT

1
```


## DELETE /v1/accounts/{account_id}/series/{series_key}/{token}/attrs/{attribute_key}

Delete a specific attribute from a Series at the specified token.

Requires `write:series` scope in the Account.

### Example request

```http
DELETE /v1/accounts/0/series/example-1/000001/attrs/first HTTP/1.1
Host: localhost:3030
Authorization: Basic NkZXa3pNeWJhdUZ3bDlrdk1kN3licEFBOlVFbUpPZkhWWjFJNmpOd2owZlV6SFZBUnpRU0FHV0VPWWZHQktTazNhQTQ4eDkwbDJFWUpuN2VJejU2ek1ORTRwRXh5TkV4WWo1NWQwcjcxb1NJanBDaTFlVlY1TFVQWW9TZGNDSklXSzNGMzd5eGJCbTJEZnNxeHNLSWd5RnND

```

### Example response

```http
HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 0
Date: Tue, 17 May 2022 22:38:16 GMT


```


## DELETE /v1/accounts/{account_id}/series/{series_key}/{token}/attrs

Delete every attribute from a Series at the specified token.

Requires `write:series` scope in the Account.

### Example request

```http
DELETE /v1/accounts/0/series/example-1/000001/attrs HTTP/1.1
Host: localhost:3030
Authorization: Basic NkZXa3pNeWJhdUZ3bDlrdk1kN3licEFBOlVFbUpPZkhWWjFJNmpOd2owZlV6SFZBUnpRU0FHV0VPWWZHQktTazNhQTQ4eDkwbDJFWUpuN2VJejU2ek1ORTRwRXh5TkV4WWo1NWQwcjcxb1NJanBDaTFlVlY1TFVQWW9TZGNDSklXSzNGMzd5eGJCbTJEZnNxeHNLSWd5RnND

```

### Example response

```http
HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 0
Date: Tue, 17 May 2022 22:38:16 GMT


```


