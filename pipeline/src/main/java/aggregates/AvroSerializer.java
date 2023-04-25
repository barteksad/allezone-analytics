package aggregates;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.util.Map;

import org.apache.avro.Schema;
import org.apache.avro.io.Encoder;
import org.apache.avro.io.EncoderFactory;
import org.apache.avro.specific.SpecificDatumWriter;
import org.apache.kafka.common.serialization.Serializer;

public class AvroSerializer<T extends org.apache.avro.specific.SpecificRecordBase & org.apache.avro.specific.SpecificRecord> implements Serializer<T> {

  private Schema schema;

  @Override
  public void configure(Map<String, ?> configs, boolean isKey) {
    schema = (Schema) configs.get("schema");
  }

  @Override
  public byte[] serialize(String topic, T data) {
    SpecificDatumWriter<T> writer = new SpecificDatumWriter<>(schema);
    ByteArrayOutputStream output = new ByteArrayOutputStream();
    Encoder encoder = EncoderFactory.get().binaryEncoder(output, null);
    try {
      writer.write(data, encoder);
      encoder.flush();
    } catch (IOException e) {
      e.printStackTrace();
    }
    return output.toByteArray();
  }
  
}
