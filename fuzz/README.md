This directory sets up running fuzz testing on both solidity and rust. We use python as the orchestration language
to (1) gather fuzz testing for solidity and rust, (2) run individual tests, and (3) log any errors from tests to
rollbar.

```
pip install -r requirements-fuzz.txt
```