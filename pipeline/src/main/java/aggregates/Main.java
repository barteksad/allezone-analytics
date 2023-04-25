package aggregates;

import java.util.Collections;
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

import allezone_analytics.AggregatesItem;
import allezone_analytics.AggregatesPrice;
import io.github.cdimascio.dotenv.Dotenv;


public class Main {

  public static void main(String[] args) {
    Dotenv dotenv = Dotenv.load();

    Serde<AggregatesItem> keyAvroSerde = Serdes.serdeFrom(new AvroSerializer<AggregatesItem>(), new AvroDeserializer<AggregatesItem>());
    Serde<AggregatesPrice> valueAvroSerde = Serdes.serdeFrom(new AvroSerializer<AggregatesPrice>(), new AvroDeserializer<AggregatesPrice>());
    keyAvroSerde.configure(Collections.singletonMap("schema", AggregatesItem.SCHEMA$), true);
    valueAvroSerde.configure(Collections.singletonMap("schema", AggregatesPrice.SCHEMA$), false);

    Properties props = new Properties();
    props.put(StreamsConfig.APPLICATION_ID_CONFIG, dotenv.get("KAFKA_STREAMS_APP_ID"));
    props.put(StreamsConfig.BOOTSTRAP_SERVERS_CONFIG, dotenv.get("KAFKA_HOSTS"));
    props.put(StreamsConfig.DEFAULT_KEY_SERDE_CLASS_CONFIG, keyAvroSerde.getClass().getName());
    props.put(StreamsConfig.DEFAULT_VALUE_SERDE_CLASS_CONFIG, valueAvroSerde.getClass().getName());

    final StoreBuilder<KeyValueStore<AggregatesItem, Long>> priceSum = Stores.keyValueStoreBuilder(
        Stores.persistentKeyValueStore("aggregates-price-sum"),
        keyAvroSerde,
        Serdes.Long()
    );

    final StoreBuilder<KeyValueStore<AggregatesItem, Long>> priceCount = Stores.keyValueStoreBuilder(
        Stores.persistentKeyValueStore("aggregates-price-count"),
        keyAvroSerde,
        Serdes.Long()
    );

    final Topology topology = new Topology();
    topology.addSource("Source", keyAvroSerde.deserializer(), valueAvroSerde.deserializer(), dotenv.get("KAFKA_TOPIC"))
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
