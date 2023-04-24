# curl -X DELETE http://localhost:8081/subjects/aggregates-events-value
# curl -X DELETE http://localhost:8081/subjects/aggregates-events-key

curl -X POST -H "Content-Type: application/vnd.schemaregistry.v1+json" \
   --data '{"schema": "{ \"namespace\": \"allezone_analytics\", \"type\": \"record\", \"name\": \"AggregatesItem\", \"fields\": [ { \"name\": \"time\", \"type\": { \"type\": \"long\", \"logicalType\": \"timestamp-millis\" } }, { \"name\": \"origin\", \"type\": \"string\" }, { \"name\": \"brand_id\", \"type\": \"string\" }, { \"name\": \"category_id\", \"type\": \"string\" } ] }" }' \
  http://localhost:8081/subjects/aggregates-events-key/versions

curl -X POST -H "Content-Type: application/vnd.schemaregistry.v1+json" \
   --data '{"schema": "{ \"namespace\": \"allezone_analytics\", \"type\": \"record\", \"name\": \"AggregatesPrice\", \"fields\": [ { \"name\": \"price\", \"type\": \"int\" } ] }" }' \
  http://localhost:8081/subjects/aggregates-events-value/versions
