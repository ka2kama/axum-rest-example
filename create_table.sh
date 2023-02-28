#!/usr/bin/env bash
set -eu

export AWS_ACCESS_KEY_ID=dummy-key
export AWS_SECRET_ACCESS_KEY=dummy-secret
export AWS_DEFAULT_REGION=ap-northeast-1

endpoint_url=http://localhost:8000

aws dynamodb create-table --table-name users \
    --attribute-definitions AttributeName=id,AttributeType=S \
    --key-schema AttributeName=id,KeyType=HASH \
    --provisioned-throughput ReadCapacityUnits=1,WriteCapacityUnits=1 \
    --endpoint-url "${endpoint_url}" | cat

aws dynamodb create-table --table-name books \
    --attribute-definitions AttributeName=id,AttributeType=S \
    --key-schema AttributeName=id,KeyType=HASH \
    --provisioned-throughput ReadCapacityUnits=1,WriteCapacityUnits=1 \
    --endpoint-url "${endpoint_url}" | cat
