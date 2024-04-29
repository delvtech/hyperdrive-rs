import os
import subprocess

import rollbar

# Get rollbar api token from env variables
token = os.environ['ROLLBAR_API_KEY']

# Set up rollbar
rollbar.init(
  access_token=token,
  environment='rust-fuzz',
)

# Gather all tests using cargo test
tests = subprocess.run(["cargo", "test", "fuzz_", "--", "--list"], stdout=subprocess.PIPE, check=True)
tests = tests.stdout.decode("utf-8").split("\n")

# Remove the final : test from tests
# The cargo test list also has some printouts that are not tests, so we remove those as well
tests = [o.split(": test")[0] for o in tests if "::" in o]

# Run forever
while True:
    # Loop through tests and run, while capturing failure output
    for test in tests:
        print(test)
        # We don't throw exception if underlying test fails
        output = subprocess.run(["cargo", "test", test], stdout=subprocess.PIPE, check=False)
        # If the test failed
        if output.returncode != 0:
            print(f"{test} failed")
            str_output = output.stdout.decode("utf-8")
            # Log to rollbar
            rollbar.report_message(f'Rust test {test} failed', 'error', extra_data={'output': str_output})


