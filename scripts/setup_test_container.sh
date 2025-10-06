#!/bin/bash
set -e
cd "$(dirname "$0")"
cd ..

docker build -f ./docker_test/Dockerfile -t restic-ssh-test .
cp docker_test/test_folders/test1/ -r ./temp_local/
docker run -d -v ./temp_remote/:/home/testuser/testsaves/ --name restic-ssh-test -p 2222:22 --rm restic-ssh-test

