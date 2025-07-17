#!/bin/sh

CONTAINER_NAME="task$1"
IMAGE_NAME="task$1:latest"

if [ "$(docker ps -a -q -f name=^/${CONTAINER_NAME}$)" ]; then
    if [ ! "$(docker ps -q -f name=^/${CONTAINER_NAME}$)" ]; then
        docker start $CONTAINER_NAME > /dev/null 2>&1
        docker attach $CONTAINER_NAME
    fi
else
    docker run -it --name $CONTAINER_NAME $IMAGE_NAME
fi
