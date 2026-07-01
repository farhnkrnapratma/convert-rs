# convert-rs
A simple and blazingly fast unit conversion API powered by Rust

# Usage

This is an example of how to use the API.

```bash
curl -s "http://127.0.0.1:8080/temperature?val=50&from=c&to=f" | jq
```

That command will send an API request to the server for converting
the given value (`50`) from the `celsius` unit to the `fahrenheit` unit.
The result for that command will look like:

```json
{
  "from": "c",
  "to": "f",
  "val": 50.0,
  "res": 122.0
}
```

# License

This project is licensed under the [MIT License](LICENSE).

Copyright © 2026 Farhan Kurnia Pratama
