# Monotron
Monotron is a [monotonically incrementing](https://en.wikipedia.org/wiki/Monotonic_function) value generation service. It generates values that are always one larger than the last value it generated.

## Cool, why?
This is intended to be useful for automatically versioning releases of software via CI/CD pipelines. For a given `key`, the system will always produce a value that is one larger than the last value, which you can use to generate the next sequential version number for your release.

## Ok, I'm convinced this is great. How am I using it?
Refer to the API documentation, such as it is, for details
