#!/bin/bash
set -e

if [ ! -d "/workspace/project/.git" ]; then
  echo "Cloning GitHub clone project..."
  git clone https://github.com/momanamjad/github /workspace/project
  echo "Clone complete."
else
  echo "Project already cloned, pulling latest..."
  cd /workspace/project && git pull
fi

export PROJECT_PATH=/workspace/project
export SERVER_ONLY=true

echo "Starting github-cli server..."
exec /usr/local/bin/github-cli