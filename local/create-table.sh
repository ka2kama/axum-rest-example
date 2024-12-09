#!/usr/bin/env bash
set -eu

export AWS_ACCESS_KEY_ID=dummyKey
export AWS_SECRET_ACCESS_KEY=dummySecret
export AWS_DEFAULT_REGION=ap-northeast-1

endpoint_url=http://localhost:${DYNAMODB_LOCAL_PORT}

aws dynamodb list-tables --endpoint-url "${endpoint_url}"

TABLE_NAME=users
if aws dynamodb describe-table --table-name ${TABLE_NAME} --endpoint-url "${endpoint_url}" > /dev/null 2>&1; then
    echo "Table ${TABLE_NAME} already exists. Skipping creation."
else
    aws dynamodb create-table --table-name ${TABLE_NAME} \
        --attribute-definitions AttributeName=id,AttributeType=S \
        --key-schema AttributeName=id,KeyType=HASH \
        --provisioned-throughput ReadCapacityUnits=1,WriteCapacityUnits=1 \
        --endpoint-url "${endpoint_url}" \
        | cat
    echo "Table ${TABLE_NAME} created successfully."
fi

TABLE_NAME=books
if aws dynamodb describe-table --table-name ${TABLE_NAME} --endpoint-url "${endpoint_url}" > /dev/null 2>&1; then
    echo "Table ${TABLE_NAME} already exists. Skipping creation."
else
    aws dynamodb create-table --table-name ${TABLE_NAME} \
        --attribute-definitions AttributeName=id,AttributeType=S \
        --key-schema AttributeName=id,KeyType=HASH \
        --provisioned-throughput ReadCapacityUnits=1,WriteCapacityUnits=1 \
        --endpoint-url "${endpoint_url}" \
        | cat
    echo "Table ${TABLE_NAME} created successfully."
fi
