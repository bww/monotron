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
Content-Length: 26

[
  "read,write:series"
]
```

### Example response

```http
HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 239
Date: Tue, 03 May 2022 00:14:04 GMT

{
  "account_id": 0,
  "api_key": {
    "id": 2,
    "key": "NKdMa4LalxMQpLowbinDmoNX",
    "secret": "SG2yX94lgp3y4PaHrSQ4gQkv3X0mMNmz7hBOLCRcQNYjRwODIX5YFKzZao5d1z6Wx7UN4DtY7sl9LNXhyuvyLTsvkKh7OPOHKBGn63d1W2KX8whwwMhz8AAoStnMDaZs"
  },
  "scopes": [
    "read,write:series"
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
Authorization: Basic VXRnZURCM3V3eXlxOUlhdlFsTzNEMUlUOnBQbEJhVlNjQUNHeXZyZFVLRXB2dVR5QTQ1dXNScllYS2g3VUZVM0pKWDZlNDhHRWlkRnpXZ0FRY0pHNnl6VGU5aEM0Qkt3Y090NGZaZjJUVzJyQWl1TGVpU1N6VEFVajJVTlBGTmc3Tk9hSFF0aWhPSGVyV2ZWamVNYVRUbnZD

```

### Example response

```http
HTTP/1.1 200 OK
Date: Tue, 03 May 2022 00:14:04 GMT
Content-Type: application/json
Content-Length: 706

[
  {
    "account_id": 0,
    "api_key": {
      "id": 2,
      "key": "NKdMa4LalxMQpLowbinDmoNX",
      "secret": "SG2yX94lgp3y4PaHrSQ4gQkv3X0mMNmz7hBOLCRcQNYjRwODIX5YFKzZao5d1z6Wx7UN4DtY7sl9LNXhyuvyLTsvkKh7OPOHKBGn63d1W2KX8whwwMhz8AAoStnMDaZs"
    },
    "scopes": [
      "read,write:series"
    ]
  },
  {
    "account_id": 0,
    "api_key": {
      "id": 3,
      "key": "UtgeDB3uwyyq9IavQlO3D1IT",
      "secret": "pPlBaVScACGyvrdUKEpvuTyA45usRrYXKh7UFU3JJX6e48GEidFzWgAQcJG6yzTe9hC4BKwcOt4fZf2TW2rAiuLeiSSzTAUj2UNPFNg7NOaHQtihOHerWfVjeMaTTnvC"
    },
    "scopes": [
      "read,write:acl"
    ]
  },
  {
    "account_id": 0,
    "api_key": {
      "id": 4,
      "key": "jYr1MBYVU87pM2Kanipn27Yg",
      "secret": "0IgXhZxwpnezJvSvHZh55x3CgAOiCfGOKKh1NTI6BVj0HQb5pEKsLKkbFC0abC5HESMlk76CS1hTJwBF8pC1dAozQcofohsg82idIPCnqXeJvBOLhcgYYQs14QKZpZt5"
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
GET /v1/accounts/0/grants/NKdMa4LalxMQpLowbinDmoNX HTTP/1.1
Host: localhost:3030
Authorization: Basic VXRnZURCM3V3eXlxOUlhdlFsTzNEMUlUOnBQbEJhVlNjQUNHeXZyZFVLRXB2dVR5QTQ1dXNScllYS2g3VUZVM0pKWDZlNDhHRWlkRnpXZ0FRY0pHNnl6VGU5aEM0Qkt3Y090NGZaZjJUVzJyQWl1TGVpU1N6VEFVajJVTlBGTmc3Tk9hSFF0aWhPSGVyV2ZWamVNYVRUbnZD
Content-Type: application/json

```

### Example response

```http
HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 239
Date: Tue, 03 May 2022 00:14:04 GMT

{
  "account_id": 0,
  "api_key": {
    "id": 2,
    "key": "NKdMa4LalxMQpLowbinDmoNX",
    "secret": "SG2yX94lgp3y4PaHrSQ4gQkv3X0mMNmz7hBOLCRcQNYjRwODIX5YFKzZao5d1z6Wx7UN4DtY7sl9LNXhyuvyLTsvkKh7OPOHKBGn63d1W2KX8whwwMhz8AAoStnMDaZs"
  },
  "scopes": [
    "read,write:series"
  ]
}
```


## DELETE /v1/accounts/{account_id}/grants/{api_key}

Delete an API Key in the specified Account.

Requires `delete:acl` scope in the Account.

### Example request

```http
DELETE /v1/accounts/0/grants/NKdMa4LalxMQpLowbinDmoNX HTTP/1.1
Host: localhost:3030
Content-Type: application/json
Authorization: Basic allyMU1CWVZVODdwTTJLYW5pcG4yN1lnOjBJZ1hoWnh3cG5lekp2U3ZIWmg1NXgzQ2dBT2lDZkdPS0toMU5USTZCVmowSFFiNXBFS3NMS2tiRkMwYWJDNUhFU01sazc2Q1MxaFRKd0JGOHBDMWRBb3pRY29mb2hzZzgyaWRJUENucVhlSnZCT0xoY2dZWVFzMTRRS1pwWnQ1

```

### Example response

```http
HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 0
Date: Tue, 03 May 2022 00:14:04 GMT


```


