# Monotron
Monotron is a [monotonically incrementing](https://en.wikipedia.org/wiki/Monotonic_function) value generation service. It generates values that are always one larger than the last value it generated.

## Great, why?
This is intended to be useful for automatically versioning releases of software via CI/CD pipelines. For a given `key`, the system will always produce a value that is one larger than the last value, which you can use to generate the next sequential version number for your release.

## I'm convinced this is great. How am I using it?
This service only has a couple endpoints. The main one you'll use requests the next value in the series for a specific key and token:

```
PUT /v1/series/{key}/{token}

{
  "creator_id": 1,
  "key": "{key}",
  "token": "{token}",
  "value": 123
}
```

The `key` is the name of the series we're operating on. Each series starts at `1` and goes to, at most, `9223372036854775807`.

If you wanted the next release number for your service named `some-great-service` which currently has the git hash `7d4c379588`, you might make the following requst;

```
PUT /v1/series/some-great-service/7d4c379588

{
  "creator_id": 1,
  "key": "some-great-service",
  "token": "7d4c379588",
  "value": 1
}
```

The `token` is any arbitrary value that you want to use to represent the current state. This is intended to be something like the git hash for the release you're making but technically it could be anything.

If you repeat the request with _the same `token`_ you will get the same result each time. This means that if for some reason you must repeate your build you will still get the same value back as log as the code (and therefore the git hash) hasn't changed.

```
PUT /v1/series/some-great-service/5f4ae5d23a

{
  "creator_id": 1,
  "key": "some-great-service",
  "token": "5f4ae5d23a",
  "value": 1
}
```

If you make the request again with _a new `token`_ you will get the next value.

```
PUT /v1/series/some-great-service/7d4c379588

{
  "creator_id": 1,
  "key": "some-great-service",
  "token": "7d4c379588",
  "value": 2
}
```

However, Monotron does not keep track of all past tokens, only the current one, so repeating the request with an older token will produce a new value.
