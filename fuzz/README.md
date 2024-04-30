This directory sets up running rust fuzz tests. We use python as the orchestration language
to (1) gather fuzz testing for rust, (2) run individual tests, and (3) log any errors from tests to
rollbar.

From the `hyperdrive-rs` root directory:

```
pip install -r fuzz/requirements-fuzz.txt
ROLLBAR_API_KEY=<rollbar_api_key> python fuzz/run-rust-fuzz.py
```