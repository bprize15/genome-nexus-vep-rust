#!/bin/bash

# This script is generated automatically by init_vep.sh - do not modify

docker exec vep-release_112.0 vep "$@" || {
    echo "Command failed. Try running './scripts/init-vep.sh' to create the container or 'docker container start vep-release_112.0' if it is stopped."
    exit 1
}
