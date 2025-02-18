#!/bin/bash

# Change directory to root Server directory for correct Docker Context
cd "$(dirname "$0")"
cd ../..

# Define an array of services and their Dockerfiles
declare -A services=(
  ["bluewaveuptime/uptime_client"]="./Docker/dist/client.Dockerfile"
  ["bluewaveuptime/uptime_database_mongo"]="./Docker/dist/mongoDB.Dockerfile"
  ["bluewaveuptime/uptime_redis"]="./Docker/dist/redis.Dockerfile"
  ["bluewaveuptime/uptime_server"]="./Docker/dist/server.Dockerfile"
)

# Loop through each service and build the corresponding image
for service in "${!services[@]}"; do
  docker build -f "${services[$service]}" -t "$service" .
  
  # Check if the build succeeded
  if [ $? -ne 0 ]; then
    echo "Error building $service image. Exiting..."
    exit 1
  fi
done

echo "All images built successfully"