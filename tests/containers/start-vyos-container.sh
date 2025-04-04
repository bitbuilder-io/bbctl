#!/bin/bash
# Start a VyOS test container for bbctl development

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
IMAGE_NAME="vyos-test"
CONTAINER_NAME="vyos-test"

# Check if Docker is installed
if ! command -v docker &> /dev/null
then
    echo "Error: Docker is not installed. Please install Docker first."
    exit 1
fi

# Build the Docker image if it doesn't exist
if ! docker image inspect $IMAGE_NAME &> /dev/null; then
    echo "Building VyOS test container image..."
    docker build -t $IMAGE_NAME $SCRIPT_DIR
fi

# Check if container already exists
if docker ps -a --format '{{.Names}}' | grep -q "^$CONTAINER_NAME$"; then
    echo "Container $CONTAINER_NAME already exists."
    
    # Check if it's running
    if docker ps --format '{{.Names}}' | grep -q "^$CONTAINER_NAME$"; then
        echo "Container is already running."
    else
        echo "Starting existing container..."
        docker start $CONTAINER_NAME
    fi
else
    echo "Creating and starting new VyOS test container..."
    
    # Create a new container
    docker run -d \
        --name $CONTAINER_NAME \
        --hostname vyos-test \
        -p 60022:22 \
        -p 60443:443 \
        -v $SCRIPT_DIR/vyos-config.boot:/etc/vyos/config/config.boot \
        $IMAGE_NAME
fi

# Display information
CONTAINER_IP=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' $CONTAINER_NAME)
echo ""
echo "VyOS test container is running!"
echo "--------------------------------"
echo "Container name: $CONTAINER_NAME"
echo "Container IP: $CONTAINER_IP"
echo "SSH access: ssh -p 60022 vyos@localhost (password: vyos)"
echo "API access: https://localhost:60443/api/ (key: test-api-key)"
echo ""
echo "To connect to the container shell:"
echo "docker exec -it $CONTAINER_NAME /bin/bash"
echo ""
echo "To stop the container:"
echo "docker stop $CONTAINER_NAME"
echo ""