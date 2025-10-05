#!/bin/bash
# Fix permissions for mounted folders
chown -R testuser:testuser /home/testuser/testsaves
exec "$@"