/**
 * Autogenerated by Avro
 *
 * DO NOT EDIT DIRECTLY
 */
package allezone_analytics;

import org.apache.avro.generic.GenericArray;
import org.apache.avro.specific.SpecificData;
import org.apache.avro.util.Utf8;
import org.apache.avro.message.BinaryMessageEncoder;
import org.apache.avro.message.BinaryMessageDecoder;
import org.apache.avro.message.SchemaStore;

@org.apache.avro.specific.AvroGenerated
public class AggregatesPrice extends org.apache.avro.specific.SpecificRecordBase implements org.apache.avro.specific.SpecificRecord {
  private static final long serialVersionUID = 3069278221055529748L;


  public static final org.apache.avro.Schema SCHEMA$ = new org.apache.avro.Schema.Parser().parse("{\"type\":\"record\",\"name\":\"AggregatesPrice\",\"namespace\":\"allezone_analytics\",\"fields\":[{\"name\":\"price\",\"type\":\"int\"}]}");
  public static org.apache.avro.Schema getClassSchema() { return SCHEMA$; }

  private static final SpecificData MODEL$ = new SpecificData();

  private static final BinaryMessageEncoder<AggregatesPrice> ENCODER =
      new BinaryMessageEncoder<>(MODEL$, SCHEMA$);

  private static final BinaryMessageDecoder<AggregatesPrice> DECODER =
      new BinaryMessageDecoder<>(MODEL$, SCHEMA$);

  /**
   * Return the BinaryMessageEncoder instance used by this class.
   * @return the message encoder used by this class
   */
  public static BinaryMessageEncoder<AggregatesPrice> getEncoder() {
    return ENCODER;
  }

  /**
   * Return the BinaryMessageDecoder instance used by this class.
   * @return the message decoder used by this class
   */
  public static BinaryMessageDecoder<AggregatesPrice> getDecoder() {
    return DECODER;
  }

  /**
   * Create a new BinaryMessageDecoder instance for this class that uses the specified {@link SchemaStore}.
   * @param resolver a {@link SchemaStore} used to find schemas by fingerprint
   * @return a BinaryMessageDecoder instance for this class backed by the given SchemaStore
   */
  public static BinaryMessageDecoder<AggregatesPrice> createDecoder(SchemaStore resolver) {
    return new BinaryMessageDecoder<>(MODEL$, SCHEMA$, resolver);
  }

  /**
   * Serializes this AggregatesPrice to a ByteBuffer.
   * @return a buffer holding the serialized data for this instance
   * @throws java.io.IOException if this instance could not be serialized
   */
  public java.nio.ByteBuffer toByteBuffer() throws java.io.IOException {
    return ENCODER.encode(this);
  }

  /**
   * Deserializes a AggregatesPrice from a ByteBuffer.
   * @param b a byte buffer holding serialized data for an instance of this class
   * @return a AggregatesPrice instance decoded from the given buffer
   * @throws java.io.IOException if the given bytes could not be deserialized into an instance of this class
   */
  public static AggregatesPrice fromByteBuffer(
      java.nio.ByteBuffer b) throws java.io.IOException {
    return DECODER.decode(b);
  }

  private int price;

  /**
   * Default constructor.  Note that this does not initialize fields
   * to their default values from the schema.  If that is desired then
   * one should use <code>newBuilder()</code>.
   */
  public AggregatesPrice() {}

  /**
   * All-args constructor.
   * @param price The new value for price
   */
  public AggregatesPrice(java.lang.Integer price) {
    this.price = price;
  }

  @Override
  public org.apache.avro.specific.SpecificData getSpecificData() { return MODEL$; }

  @Override
  public org.apache.avro.Schema getSchema() { return SCHEMA$; }

  // Used by DatumWriter.  Applications should not call.
  @Override
  public java.lang.Object get(int field$) {
    switch (field$) {
    case 0: return price;
    default: throw new IndexOutOfBoundsException("Invalid index: " + field$);
    }
  }

  // Used by DatumReader.  Applications should not call.
  @Override
  @SuppressWarnings(value="unchecked")
  public void put(int field$, java.lang.Object value$) {
    switch (field$) {
    case 0: price = (java.lang.Integer)value$; break;
    default: throw new IndexOutOfBoundsException("Invalid index: " + field$);
    }
  }

  /**
   * Gets the value of the 'price' field.
   * @return The value of the 'price' field.
   */
  public int getPrice() {
    return price;
  }


  /**
   * Sets the value of the 'price' field.
   * @param value the value to set.
   */
  public void setPrice(int value) {
    this.price = value;
  }

  /**
   * Creates a new AggregatesPrice RecordBuilder.
   * @return A new AggregatesPrice RecordBuilder
   */
  public static allezone_analytics.AggregatesPrice.Builder newBuilder() {
    return new allezone_analytics.AggregatesPrice.Builder();
  }

  /**
   * Creates a new AggregatesPrice RecordBuilder by copying an existing Builder.
   * @param other The existing builder to copy.
   * @return A new AggregatesPrice RecordBuilder
   */
  public static allezone_analytics.AggregatesPrice.Builder newBuilder(allezone_analytics.AggregatesPrice.Builder other) {
    if (other == null) {
      return new allezone_analytics.AggregatesPrice.Builder();
    } else {
      return new allezone_analytics.AggregatesPrice.Builder(other);
    }
  }

  /**
   * Creates a new AggregatesPrice RecordBuilder by copying an existing AggregatesPrice instance.
   * @param other The existing instance to copy.
   * @return A new AggregatesPrice RecordBuilder
   */
  public static allezone_analytics.AggregatesPrice.Builder newBuilder(allezone_analytics.AggregatesPrice other) {
    if (other == null) {
      return new allezone_analytics.AggregatesPrice.Builder();
    } else {
      return new allezone_analytics.AggregatesPrice.Builder(other);
    }
  }

  /**
   * RecordBuilder for AggregatesPrice instances.
   */
  @org.apache.avro.specific.AvroGenerated
  public static class Builder extends org.apache.avro.specific.SpecificRecordBuilderBase<AggregatesPrice>
    implements org.apache.avro.data.RecordBuilder<AggregatesPrice> {

    private int price;

    /** Creates a new Builder */
    private Builder() {
      super(SCHEMA$, MODEL$);
    }

    /**
     * Creates a Builder by copying an existing Builder.
     * @param other The existing Builder to copy.
     */
    private Builder(allezone_analytics.AggregatesPrice.Builder other) {
      super(other);
      if (isValidValue(fields()[0], other.price)) {
        this.price = data().deepCopy(fields()[0].schema(), other.price);
        fieldSetFlags()[0] = other.fieldSetFlags()[0];
      }
    }

    /**
     * Creates a Builder by copying an existing AggregatesPrice instance
     * @param other The existing instance to copy.
     */
    private Builder(allezone_analytics.AggregatesPrice other) {
      super(SCHEMA$, MODEL$);
      if (isValidValue(fields()[0], other.price)) {
        this.price = data().deepCopy(fields()[0].schema(), other.price);
        fieldSetFlags()[0] = true;
      }
    }

    /**
      * Gets the value of the 'price' field.
      * @return The value.
      */
    public int getPrice() {
      return price;
    }


    /**
      * Sets the value of the 'price' field.
      * @param value The value of 'price'.
      * @return This builder.
      */
    public allezone_analytics.AggregatesPrice.Builder setPrice(int value) {
      validate(fields()[0], value);
      this.price = value;
      fieldSetFlags()[0] = true;
      return this;
    }

    /**
      * Checks whether the 'price' field has been set.
      * @return True if the 'price' field has been set, false otherwise.
      */
    public boolean hasPrice() {
      return fieldSetFlags()[0];
    }


    /**
      * Clears the value of the 'price' field.
      * @return This builder.
      */
    public allezone_analytics.AggregatesPrice.Builder clearPrice() {
      fieldSetFlags()[0] = false;
      return this;
    }

    @Override
    @SuppressWarnings("unchecked")
    public AggregatesPrice build() {
      try {
        AggregatesPrice record = new AggregatesPrice();
        record.price = fieldSetFlags()[0] ? this.price : (java.lang.Integer) defaultValue(fields()[0]);
        return record;
      } catch (org.apache.avro.AvroMissingFieldException e) {
        throw e;
      } catch (java.lang.Exception e) {
        throw new org.apache.avro.AvroRuntimeException(e);
      }
    }
  }

  @SuppressWarnings("unchecked")
  private static final org.apache.avro.io.DatumWriter<AggregatesPrice>
    WRITER$ = (org.apache.avro.io.DatumWriter<AggregatesPrice>)MODEL$.createDatumWriter(SCHEMA$);

  @Override public void writeExternal(java.io.ObjectOutput out)
    throws java.io.IOException {
    WRITER$.write(this, SpecificData.getEncoder(out));
  }

  @SuppressWarnings("unchecked")
  private static final org.apache.avro.io.DatumReader<AggregatesPrice>
    READER$ = (org.apache.avro.io.DatumReader<AggregatesPrice>)MODEL$.createDatumReader(SCHEMA$);

  @Override public void readExternal(java.io.ObjectInput in)
    throws java.io.IOException {
    READER$.read(this, SpecificData.getDecoder(in));
  }

  @Override protected boolean hasCustomCoders() { return true; }

  @Override public void customEncode(org.apache.avro.io.Encoder out)
    throws java.io.IOException
  {
    out.writeInt(this.price);

  }

  @Override public void customDecode(org.apache.avro.io.ResolvingDecoder in)
    throws java.io.IOException
  {
    org.apache.avro.Schema.Field[] fieldOrder = in.readFieldOrderIfDiff();
    if (fieldOrder == null) {
      this.price = in.readInt();

    } else {
      for (int i = 0; i < 1; i++) {
        switch (fieldOrder[i].pos()) {
        case 0:
          this.price = in.readInt();
          break;

        default:
          throw new java.io.IOException("Corrupt ResolvingDecoder.");
        }
      }
    }
  }
}










