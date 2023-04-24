package aggregates;

import java.util.Collections;
import java.util.Map;
import java.util.Properties;
import java.util.concurrent.CountDownLatch;

import org.apache.kafka.common.serialization.Serde;
import org.apache.kafka.common.serialization.Serdes;
import org.apache.kafka.streams.KafkaStreams;
import org.apache.kafka.streams.StreamsConfig;
import org.apache.kafka.streams.Topology;
import org.apache.kafka.streams.state.KeyValueStore;
import org.apache.kafka.streams.state.StoreBuilder;
import org.apache.kafka.streams.state.Stores;

import io.confluent.kafka.streams.serdes.avro.SpecificAvroSerde;

import allezone_analytics.AggregatesPrice;
import allezone_analytics.AggregatesItem;
import io.github.cdimascio.dotenv.Dotenv;


public class Main {

  public static void main(String[] args) {
    Dotenv dotenv = Dotenv.load();

    Properties props = new Properties();
    props.put(StreamsConfig.APPLICATION_ID_CONFIG, dotenv.get("KAFKA_STREAMS_APP_ID"));
    props.put(StreamsConfig.BOOTSTRAP_SERVERS_CONFIG, dotenv.get("KAFKA_HOSTS"));
    props.put(StreamsConfig.DEFAULT_KEY_SERDE_CLASS_CONFIG, SpecificAvroSerde.class);
    props.put(StreamsConfig.DEFAULT_VALUE_SERDE_CLASS_CONFIG, SpecificAvroSerde.class);
    props.put("schema.registry.url", dotenv.get("SCHEMA_REGISTRY_URL"));

    final Map<String, String> serdeConfig = Collections.singletonMap("schema.registry.url", dotenv.get("SCHEMA_REGISTRY_URL"));

    final Serde<AggregatesItem> keySpecificAvroSerde = new SpecificAvroSerde<>();
    keySpecificAvroSerde.configure(serdeConfig, true);
    final Serde<AggregatesPrice> valueSpecificAvroSerde = new SpecificAvroSerde<>();
    valueSpecificAvroSerde.configure(serdeConfig, false);

    final StoreBuilder<KeyValueStore<AggregatesItem, Long>> priceSum = Stores.keyValueStoreBuilder(
        Stores.persistentKeyValueStore("aggregates-price-sum"),
        Serdes.serdeFrom(keySpecificAvroSerde.serializer(), keySpecificAvroSerde.deserializer()),
        Serdes.Long()
    );

    final StoreBuilder<KeyValueStore<AggregatesItem, Long>> priceCount = Stores.keyValueStoreBuilder(
        Stores.persistentKeyValueStore("aggregates-price-count"),
        Serdes.serdeFrom(keySpecificAvroSerde.serializer(), keySpecificAvroSerde.deserializer()),
        Serdes.Long()
    );

    final Topology topology = new Topology();
    topology.addSource("Source", keySpecificAvroSerde.deserializer(), valueSpecificAvroSerde.deserializer(), dotenv.get("KAFKA_TOPIC"))
        .addProcessor("processor", () -> new AggregatesProcessor(), "Source")
        .addStateStore(priceSum, "processor")
        .addStateStore(priceCount, "processor");
    
    final KafkaStreams streams = new KafkaStreams(topology, props);
    final CountDownLatch latch = new CountDownLatch(1);

    Runtime.getRuntime().addShutdownHook(new Thread("streams-shutdown-hook") {
        @Override
        public void run() {
            streams.close();
            latch.countDown();
        }
    });

    try {
        streams.start();
        latch.await();
    } catch (Throwable e) {
        System.exit(1);
    }
    System.exit(0);
  
}
}
