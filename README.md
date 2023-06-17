## Services structure: 
### &emsp;  API gateway:
&emsp; &emsp; Rust REST server written using Axum and Tokio asynchronus runtime. 
### &emsp; Aerospike database: 
&emsp; &emsp; Used for storing users profiles infos as ordered by timestamp map 
### &emsp; Kafka: 
&emsp; &emsp; Used for storing events stream for further processing. 
\
&emsp; &emsp; Tuple (timestamp rounded to minutes,action, origin, brand_id, category_id) is used as key for partitioning 
\
&emsp; &emsp; There are two partitions since there are also two stream processing apps.
\
&emsp; &emsp; Docker image wurstmeister/kafka is used for automatic cluster deployment 
### &emsp; Kafka Streams Java app: 
&emsp; &emsp; Stream processing app which calculates items sum price and count and saves it in relational database. 
&emsp; &emsp; 
### &emsp; Postgress Patroni cluster: 
&emsp; &emsp; Used for storing precalculated aggregates data. 
### &emsp; Prometheus and Grafana: 
&emsp; &emsp; Used for monitoring. 
### &emsp; Apache Avro:
&emsp; &emsp; Used for serializing/deserializing data between services.

## Deployment: 
&emsp; Docker swarm on all 10 vm-s is used for cluster deployment.
\
&emsp; Loadbalancing for API gateway is also handeled by docker. By default it uses Round-Robin.
\
&emsp; Because there are more than 10 services after replication, node labels and deployment constraints are introduced 
\
&emsp; to make sure for example that no two database services are running on the same machine 
\
&emsp; which could downgrade performance and in long run could lead to out of disc space. 
\
\
&emsp; Deployment configuration is splitted into 2 files: 
\
&emsp; &emsp; 1.docker-compose.yaml - stateful services: aerospike, kafka, postgres
\
&emsp; &emsp; 2.docker-compose.yaml - stateless services: api-gateway, kafka-streams, monitoring: prometheus, grafana

Before deploying, make sure to have nodes with at least those labels, or delete them from docker-compose.yml: 
```
[st118vm101]: map[service:gateway]
[st118vm102]: map[service:gateway]
[st118vm103]: map[service:aerospike1]
[st118vm104]: map[service:aerospike2]
[st118vm105]: map[service:patroni1]
[st118vm106]: map[service:kafka]
[st118vm107]: map[service:kafka]
[st118vm108]: map[service:pipeline]
[st118vm109]: map[service:pipeline]
[st118vm110]: map[service:patroni2]
```
to deploy run: 
```
docker build -t st118vm101.rtb-lab.pl/gateway:debug -f gateway/Dockerfile.debug gateway
docker build -t st118vm101.rtb-lab.pl/pipeline:latest pipeline

docker push st118vm101.rtb-lab.pl/gateway:debug
docker push st118vm101.rtb-lab.pl/pipeline:latest

docker stack deploy -c 1.docker-compose.yaml my_stack
psql --host 127.0.0.1 --port 5000 -U postgres < services/postgres/init.sql
docker stack deploy -c 2.docker-compose.yaml my_stack
```
## Debug mode: 
\
API gateway can be compiled with debug mode enabled, which will compare received results.
\
It can be disabled by not specifying --features query-debug flag in docker-compose.yml.
\
Then the REST endpoint will be compiled without ground true body with json response.
eg:
```
pub async fn user_profiles(
    ...
    #[cfg(feature = "query-debug")] body: Json<UserProfilesResponse>,
) 

pub async fn aggregates(
    ...
    #[cfg(feature = "query-debug")] body: Json<AggregatesResponse>,
)
```
