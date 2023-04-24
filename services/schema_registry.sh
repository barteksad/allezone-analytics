docker run -d \
  --net=host \
  --name=schema-registry \
  -e SCHEMA_REGISTRY_KAFKASTORE_BOOTSTRAP_SERVERS=10.112.118.104:9092 \
  -e SCHEMA_REGISTRY_HOST_NAME=localhost \
  -e "SCHEMAREGISTRY_URL=http://localhost:8081" \
  -e SCHEMA_REGISTRY_DEBUG=true \
  confluentinc/cp-schema-registry:latest