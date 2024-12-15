#!/bin/bash
set -e

if [ $# -lt 1 ]; then
    echo "Usage: $0 TAG"
    exit 1
fi

TAG=$1
IMAGE_NAME="ensemblorg/ensembl-vep:$TAG"
CONTAINER_NAME="vep-$TAG"

# Start container in interactive mode with a persistent shell
docker run -dt \
    --name $CONTAINER_NAME \
    -v ~/.vep/cache:/opt/vep/.vep \
    -v $PWD:/data \
    $IMAGE_NAME \
    /bin/bash

# Create command passthrough script
cat > ./scripts/vep.sh << EOF
#!/bin/bash

# This script is generated automatically by init_vep.sh - do not modify

docker exec ${CONTAINER_NAME} vep "\$@" || {
    echo "Command failed. Try running './scripts/init-vep.sh' to create the container or 'docker container start $CONTAINER_NAME' if it is stopped."
    exit 1
}
EOF

echo 
echo "VEP running in container '${CONTAINER_NAME}' and is accessible from the scripts directory."
echo "Use ./vep --help to get started" 