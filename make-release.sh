#!/usr/bin/env bash
set -e

# Default version bump type
BUMP_TYPE="minor"

# Parse command line arguments
while getopts "b:" opt; do
  case $opt in
    b)
      BUMP_TYPE=$OPTARG
      ;;
    \?)
      echo "Invalid option: -$OPTARG" >&2
      exit 1
      ;;
  esac
done

cargo bump $BUMP_TYPE

NEW_VERSION=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')
git tag "$NEW_VERSION"

git add Cargo.toml
git add Cargo.lock
git commit -m "Release $NEW_VERSION"
git push
git push --tags