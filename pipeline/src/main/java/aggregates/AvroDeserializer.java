package aggregates;

import java.io.IOException;
import java.util.Map;

import org.apache.avro.Schema;
import org.apache.avro.io.BinaryDecoder;
import org.apache.avro.io.DecoderFactory;
import org.apache.avro.specific.SpecificDatumReader;
import org.apache.kafka.common.serialization.Deserializer;

public class AvroDeserializer<T extends org.apache.avro.specific.SpecificRecordBase & org.apache.avro.specific.SpecificRecord> implements Deserializer<T> {

  private Schema schema = null;

  @Override
  public void configure(Map<String, ?> configs, boolean isKey) {
    schema = (Schema) configs.get("schema");
  }
  
  @Override
  public T deserialize(String topic, byte[] data) {
    SpecificDatumReader<T> reader = new SpecificDatumReader<>(schema, schema);
    BinaryDecoder decoder = DecoderFactory.get().binaryDecoder(data, null);
    try {
      return reader.read(null, decoder);
    } catch (IOException e) {
      e.printStackTrace();
    }
    return null;
  }}
