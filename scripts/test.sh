#!/bin/bash
set -e

cd "$(dirname "$0")"
cd ..

docker build -f ./tests/Dockerfile -t restic-ssh-test .

docker run -d --name restic-ssh-test -p 2222:22 --rm restic-ssh-test
cargo test || true

docker stop restic-ssh-test