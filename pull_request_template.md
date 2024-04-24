# Resolved Issues

# Description

# Review Checklists

Please check each item **before approving** the pull request. While going
through the checklist, it is recommended to leave comments on items that are
referenced in the checklist to make sure that they are reviewed.

- [ ] **Testing**
    - [ ] Are there new or updated unit or integration tests?
    - [ ] Do the tests cover the happy paths?
    - [ ] Do the tests cover the unhappy paths?
    - [ ] Are there an adequate number of fuzz tests to ensure that we are
          covering the full input space?
    - [ ] If matching Solidity behavior, are there differential fuzz tests that
          ensure that Rust matches Solidity?
