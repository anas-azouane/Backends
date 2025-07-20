#!/bin/bash

for dir in */; do
	(cd "$dir" && docker compose up -d)
done

docker stats $(docker ps --format '{{.Names}}' | grep -E '^(springbackend-backend|spring|rocketbackend-backend|nodebackend-backend)')
