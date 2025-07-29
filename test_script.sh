#!/bin/bash

# Test script to simulate command execution and output capture

echo "Testing CCO system..."

# Simulate a command and its output
echo "ls -la" > /tmp/test_command.txt
ls -la > /tmp/test_output.txt 2>&1

# Combine them
echo "Command: $(cat /tmp/test_command.txt)"
echo "Output:"
cat /tmp/test_output.txt

# Now use cco-capture to store this
echo "Storing with cco-capture..."
cat /tmp/test_output.txt | cco-capture "$(cat /tmp/test_command.txt)"

echo "Testing cco retrieval..."
cco -p

# Cleanup
rm -f /tmp/test_command.txt /tmp/test_output.txt