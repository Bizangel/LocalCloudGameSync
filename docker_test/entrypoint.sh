#!/bin/bash
# Fix permissions for mounted folders
mkdir -p /home/testuser/testsaves/.cloudmeta
echo "test_password" > /home/testuser/testsaves/.cloudmeta/restic_password

chown -R testuser:testuser /home/testuser/testsaves
exec "$@"