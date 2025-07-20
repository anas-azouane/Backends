#!/bin/bash

for dir in */; do
	(cd "$dir" && docker compose down --rmi all --volumes --remove-orphans)
done

