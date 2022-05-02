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
Date: Mon, 02 May 2022 13:23:11 GMT
Content-Type: application/json
Content-Length: 238

{
  "account_id": 0,
  "api_key": {
    "id": 2,
    "key": "nPhhOGVyh86y4q31F0WuBgTT",
    "secret": "IBhw0dbUhOdjL4HyPQ3WpWWq6SILq9gfrujrnuLYbS0hBGLte2FpCnwQ2qGuk3CwGFWXaeTTTnodxtasDrUCIZYF7GM9B4Z8DYERbWPVriQD3FbStsmJzJStnsPd4Bif"
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
Authorization: Basic Nm9vczlxOVRRWDRRVUo1WFlHRE41N1RyOnVoUEl2UTNLUXYyNEpKQ3Vpb3lPSkFwSENSbW5nS3lTenVYQ3N3RjN5OW1lR095b05XbFN5SHhKaXlQTm05eTRLdHhjOTJKWG9wMDlRa3RDQ1pFTml1NUdkWDZaMlBOVHNSTXRiMGpyYlJBTGtnSjZkb09oSkUxVFM2M2FJNHEx

```

### Example response

```http
HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 705
Date: Mon, 02 May 2022 13:23:11 GMT

[
  {
    "account_id": 0,
    "api_key": {
      "id": 2,
      "key": "nPhhOGVyh86y4q31F0WuBgTT",
      "secret": "IBhw0dbUhOdjL4HyPQ3WpWWq6SILq9gfrujrnuLYbS0hBGLte2FpCnwQ2qGuk3CwGFWXaeTTTnodxtasDrUCIZYF7GM9B4Z8DYERbWPVriQD3FbStsmJzJStnsPd4Bif"
    },
    "scopes": [
      "read,write:entry"
    ]
  },
  {
    "account_id": 0,
    "api_key": {
      "id": 3,
      "key": "6oos9q9TQX4QUJ5XYGDN57Tr",
      "secret": "uhPIvQ3KQv24JJCuioyOJApHCRmngKySzuXCswF3y9meGOyoNWlSyHxJiyPNm9y4Ktxc92JXop09QktCCZENiu5GdX6Z2PNTsRMtb0jrbRALkgJ6doOhJE1TS63aI4q1"
    },
    "scopes": [
      "read,write:acl"
    ]
  },
  {
    "account_id": 0,
    "api_key": {
      "id": 4,
      "key": "JvsZia8TU3dKYA8q2f4h7Lyi",
      "secret": "0x8hU4IyzBzMuNh7bxBSSBvI3RhCVINx9cjc4fNWOSLwwlVlMwMOATCJGiWJqgkcia0azHqxVVTeG06tqdzGeY7vTRJfIDAIE1rw3e5YrBRpNcIRZYcOKe52T1tsoqjO"
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
GET /v1/accounts/0/grants/nPhhOGVyh86y4q31F0WuBgTT HTTP/1.1
Host: localhost:3030
Authorization: Basic Nm9vczlxOVRRWDRRVUo1WFlHRE41N1RyOnVoUEl2UTNLUXYyNEpKQ3Vpb3lPSkFwSENSbW5nS3lTenVYQ3N3RjN5OW1lR095b05XbFN5SHhKaXlQTm05eTRLdHhjOTJKWG9wMDlRa3RDQ1pFTml1NUdkWDZaMlBOVHNSTXRiMGpyYlJBTGtnSjZkb09oSkUxVFM2M2FJNHEx
Content-Type: application/json

```

### Example response

```http
HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 238
Date: Mon, 02 May 2022 13:23:11 GMT

{
  "account_id": 0,
  "api_key": {
    "id": 2,
    "key": "nPhhOGVyh86y4q31F0WuBgTT",
    "secret": "IBhw0dbUhOdjL4HyPQ3WpWWq6SILq9gfrujrnuLYbS0hBGLte2FpCnwQ2qGuk3CwGFWXaeTTTnodxtasDrUCIZYF7GM9B4Z8DYERbWPVriQD3FbStsmJzJStnsPd4Bif"
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
DELETE /v1/accounts/0/grants/nPhhOGVyh86y4q31F0WuBgTT HTTP/1.1
Host: localhost:3030
Content-Type: application/json
Authorization: Basic SnZzWmlhOFRVM2RLWUE4cTJmNGg3THlpOjB4OGhVNEl5ekJ6TXVOaDdieEJTU0J2STNSaENWSU54OWNqYzRmTldPU0x3d2xWbE13TU9BVENKR2lXSnFna2NpYTBhekhxeFZWVGVHMDZ0cWR6R2VZN3ZUUkpmSURBSUUxcnczZTVZckJScE5jSVJaWWNPS2U1MlQxdHNvcWpP

```

### Example response

```http
HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 0
Date: Mon, 02 May 2022 13:23:10 GMT


```


