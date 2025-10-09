#!/bin/bash
set -e
cd "$(dirname "$0")"
cd ..

docker build -f ./docker_test/Dockerfile -t restic-ssh-test .
mkdir -p ./temp_tests
docker run -d -v ./temp_tests/temp_remote/:/home/testuser/testsaves/ --name restic-ssh-test -p 2222:22 --rm restic-ssh-test

