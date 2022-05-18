# Monotonic Token Attributes

## Contents

* [PUT /v1/accounts/{account_id}/tokens/{series_key}/{token}/attrs](#put-v1accountsaccount_idtokensseries_keytokenattrs)
* [GET /v1/accounts/{account_id}/tokens/{series_key}/{token}/attrs](#get-v1accountsaccount_idtokensseries_keytokenattrs)
* [GET /v1/accounts/{account_id}/tokens/{series_key}/{token}/attrs/{attribute_key}](#get-v1accountsaccount_idtokensseries_keytokenattrsattribute_key)
* [DELETE /v1/accounts/{account_id}/tokens/{series_key}/{token}/attrs/{attribute_key}](#delete-v1accountsaccount_idtokensseries_keytokenattrsattribute_key)
* [DELETE /v1/accounts/{account_id}/tokens/{series_key}/{token}/attrs](#delete-v1accountsaccount_idtokensseries_keytokenattrs)

## PUT /v1/accounts/{account_id}/tokens/{series_key}/{token}/attrs

Store multiple attributes for a Series at the specified token.

This operation might be more accurately described as a `PATCH`; keys present
in the provied set are stored, but any existing keys not present in the
provided set are not deleted.

Requires `write:series` scope in the Account.

### Example request

```http
PUT /v1/accounts/0/tokens/example-1/000001/attrs HTTP/1.1
Host: localhost:3030
Authorization: Basic M1JYUVlMTEFHV0prVHV3R0FKWW9lcGEwOjBzaE9OUFJtUjNYMkNVa2NnTk1XSEx3MkNkVklHRUZVTG42NHgxVlZxWFZNbmVtSVl5RzNhODQwWTlMUTR2UnU3elRlUU13MmRzSzJETmJWdHdVRGRydlVLUGV3RUNEQ0dTejRET0NQTmcxNm1UdU9EVnJWNllEcDc0NU9jZGJP
Content-Length: 52
Content-Type: application/json

{
  "first": "1",
  "second": "2",
  "third": "3"
}
```

### Example response

```http
HTTP/1.1 200 OK
Date: Wed, 18 May 2022 12:52:15 GMT
Content-Type: application/json
Content-Length: 38

{
  "first": "1",
  "second": "2",
  "third": "3"
}
```


## GET /v1/accounts/{account_id}/tokens/{series_key}/{token}/attrs

Fetch attributes for a Series at the specified token.

Requires `read:series` scope in the Account.

### Example request

```http
GET /v1/accounts/0/tokens/example-1/000001/attrs HTTP/1.1
Host: localhost:3030
Authorization: Basic RDE3M2ZSNEdyVTc5NTNaR1BONDZCa2xLOk5Tc3M5aDE3UzNCZVVkQndRM0Z3bDNoZkE0dGkwaGh1RkJ3ZWVBQzh1VW5yODB5ZzZCemNSTUM1T1piQzV1OXZhTzRBR3RCT3JNdFU3TkJmZGl6SFNKZm1KcVlucE84c1ZxRUx3UlNMbjgwMGF0MzY1YU1mek9UcGViMGlFbnpP

```

### Example response

```http
HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 38
Date: Wed, 18 May 2022 12:52:15 GMT

{
  "first": "1",
  "second": "2",
  "third": "3"
}
```


## GET /v1/accounts/{account_id}/tokens/{series_key}/{token}/attrs/{attribute_key}

Fetch a specific attribute for a Series at the specified token.

Requires `read:series` scope in the Account.

### Example request

```http
GET /v1/accounts/0/tokens/example-1/000001/attrs/first HTTP/1.1
Host: localhost:3030
Authorization: Basic RDE3M2ZSNEdyVTc5NTNaR1BONDZCa2xLOk5Tc3M5aDE3UzNCZVVkQndRM0Z3bDNoZkE0dGkwaGh1RkJ3ZWVBQzh1VW5yODB5ZzZCemNSTUM1T1piQzV1OXZhTzRBR3RCT3JNdFU3TkJmZGl6SFNKZm1KcVlucE84c1ZxRUx3UlNMbjgwMGF0MzY1YU1mek9UcGViMGlFbnpP

```

### Example response

```http
HTTP/1.1 200 OK
Date: Wed, 18 May 2022 12:52:15 GMT
Content-Type: application/json
Content-Length: 1

1
```


## DELETE /v1/accounts/{account_id}/tokens/{series_key}/{token}/attrs/{attribute_key}

Delete a specific attribute from a Series at the specified token.

Requires `write:series` scope in the Account.

### Example request

```http
DELETE /v1/accounts/0/tokens/example-1/000001/attrs/first HTTP/1.1
Host: localhost:3030
Authorization: Basic M1JYUVlMTEFHV0prVHV3R0FKWW9lcGEwOjBzaE9OUFJtUjNYMkNVa2NnTk1XSEx3MkNkVklHRUZVTG42NHgxVlZxWFZNbmVtSVl5RzNhODQwWTlMUTR2UnU3elRlUU13MmRzSzJETmJWdHdVRGRydlVLUGV3RUNEQ0dTejRET0NQTmcxNm1UdU9EVnJWNllEcDc0NU9jZGJP

```

### Example response

```http
HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 0
Date: Wed, 18 May 2022 12:52:15 GMT


```


## DELETE /v1/accounts/{account_id}/tokens/{series_key}/{token}/attrs

Delete every attribute from a Series at the specified token.

Requires `write:series` scope in the Account.

### Example request

```http
DELETE /v1/accounts/0/tokens/example-1/000001/attrs HTTP/1.1
Host: localhost:3030
Authorization: Basic M1JYUVlMTEFHV0prVHV3R0FKWW9lcGEwOjBzaE9OUFJtUjNYMkNVa2NnTk1XSEx3MkNkVklHRUZVTG42NHgxVlZxWFZNbmVtSVl5RzNhODQwWTlMUTR2UnU3elRlUU13MmRzSzJETmJWdHdVRGRydlVLUGV3RUNEQ0dTejRET0NQTmcxNm1UdU9EVnJWNllEcDc0NU9jZGJP

```

### Example response

```http
HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 0
Date: Wed, 18 May 2022 12:52:15 GMT


```


