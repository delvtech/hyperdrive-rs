#!/bin/bash

# GitHub Token provided by GitHub Actions
TOKEN=${GITHUB_TOKEN}

# Repository from which to fetch the latest tag
REPO='delvtech/hyperdrive'

# Fetch the latest release from GitHub API
TAG=$(curl -sH "Authorization: token $TOKEN" "https://api.github.com/repos/$REPO/releases/latest" | jq -r '.tag_name')

# Output the tag for subsequent steps
echo "latest_tag=$TAG" >> $GITHUB_ENV
