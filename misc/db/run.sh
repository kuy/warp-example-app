#! /bin/bash

docker run --name warp-db \
    -p 3306:3306 \
    -e MYSQL_DATABASE=warpexample \
    -e MYSQL_USER=warpexample \
    -e MYSQL_PASSWORD=warpexample \
    -e MYSQL_ROOT_PASSWORD=root \
    -d "warp-db:1.0"
