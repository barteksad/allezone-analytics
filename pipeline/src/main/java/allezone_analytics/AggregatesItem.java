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
public class AggregatesItem extends org.apache.avro.specific.SpecificRecordBase implements org.apache.avro.specific.SpecificRecord {
  private static final long serialVersionUID = 3347091981932601970L;


  public static final org.apache.avro.Schema SCHEMA$ = new org.apache.avro.Schema.Parser().parse("{\"type\":\"record\",\"name\":\"AggregatesItem\",\"namespace\":\"allezone_analytics\",\"fields\":[{\"name\":\"time\",\"type\":{\"type\":\"long\",\"logicalType\":\"timestamp-millis\"}},{\"name\":\"origin\",\"type\":\"string\"},{\"name\":\"brand_id\",\"type\":\"string\"},{\"name\":\"category_id\",\"type\":\"string\"}]}");
  public static org.apache.avro.Schema getClassSchema() { return SCHEMA$; }

  private static final SpecificData MODEL$ = new SpecificData();
  static {
    MODEL$.addLogicalTypeConversion(new org.apache.avro.data.TimeConversions.TimestampMillisConversion());
  }

  private static final BinaryMessageEncoder<AggregatesItem> ENCODER =
      new BinaryMessageEncoder<>(MODEL$, SCHEMA$);

  private static final BinaryMessageDecoder<AggregatesItem> DECODER =
      new BinaryMessageDecoder<>(MODEL$, SCHEMA$);

  /**
   * Return the BinaryMessageEncoder instance used by this class.
   * @return the message encoder used by this class
   */
  public static BinaryMessageEncoder<AggregatesItem> getEncoder() {
    return ENCODER;
  }

  /**
   * Return the BinaryMessageDecoder instance used by this class.
   * @return the message decoder used by this class
   */
  public static BinaryMessageDecoder<AggregatesItem> getDecoder() {
    return DECODER;
  }

  /**
   * Create a new BinaryMessageDecoder instance for this class that uses the specified {@link SchemaStore}.
   * @param resolver a {@link SchemaStore} used to find schemas by fingerprint
   * @return a BinaryMessageDecoder instance for this class backed by the given SchemaStore
   */
  public static BinaryMessageDecoder<AggregatesItem> createDecoder(SchemaStore resolver) {
    return new BinaryMessageDecoder<>(MODEL$, SCHEMA$, resolver);
  }

  /**
   * Serializes this AggregatesItem to a ByteBuffer.
   * @return a buffer holding the serialized data for this instance
   * @throws java.io.IOException if this instance could not be serialized
   */
  public java.nio.ByteBuffer toByteBuffer() throws java.io.IOException {
    return ENCODER.encode(this);
  }

  /**
   * Deserializes a AggregatesItem from a ByteBuffer.
   * @param b a byte buffer holding serialized data for an instance of this class
   * @return a AggregatesItem instance decoded from the given buffer
   * @throws java.io.IOException if the given bytes could not be deserialized into an instance of this class
   */
  public static AggregatesItem fromByteBuffer(
      java.nio.ByteBuffer b) throws java.io.IOException {
    return DECODER.decode(b);
  }

  private java.time.Instant time;
  private java.lang.CharSequence origin;
  private java.lang.CharSequence brand_id;
  private java.lang.CharSequence category_id;

  /**
   * Default constructor.  Note that this does not initialize fields
   * to their default values from the schema.  If that is desired then
   * one should use <code>newBuilder()</code>.
   */
  public AggregatesItem() {}

  /**
   * All-args constructor.
   * @param time The new value for time
   * @param origin The new value for origin
   * @param brand_id The new value for brand_id
   * @param category_id The new value for category_id
   */
  public AggregatesItem(java.time.Instant time, java.lang.CharSequence origin, java.lang.CharSequence brand_id, java.lang.CharSequence category_id) {
    this.time = time.truncatedTo(java.time.temporal.ChronoUnit.MILLIS);
    this.origin = origin;
    this.brand_id = brand_id;
    this.category_id = category_id;
  }

  @Override
  public org.apache.avro.specific.SpecificData getSpecificData() { return MODEL$; }

  @Override
  public org.apache.avro.Schema getSchema() { return SCHEMA$; }

  // Used by DatumWriter.  Applications should not call.
  @Override
  public java.lang.Object get(int field$) {
    switch (field$) {
    case 0: return time;
    case 1: return origin;
    case 2: return brand_id;
    case 3: return category_id;
    default: throw new IndexOutOfBoundsException("Invalid index: " + field$);
    }
  }

  private static final org.apache.avro.Conversion<?>[] conversions =
      new org.apache.avro.Conversion<?>[] {
      new org.apache.avro.data.TimeConversions.TimestampMillisConversion(),
      null,
      null,
      null,
      null
  };

  @Override
  public org.apache.avro.Conversion<?> getConversion(int field) {
    return conversions[field];
  }

  // Used by DatumReader.  Applications should not call.
  @Override
  @SuppressWarnings(value="unchecked")
  public void put(int field$, java.lang.Object value$) {
    switch (field$) {
    case 0: time = (java.time.Instant)value$; break;
    case 1: origin = (java.lang.CharSequence)value$; break;
    case 2: brand_id = (java.lang.CharSequence)value$; break;
    case 3: category_id = (java.lang.CharSequence)value$; break;
    default: throw new IndexOutOfBoundsException("Invalid index: " + field$);
    }
  }

  /**
   * Gets the value of the 'time' field.
   * @return The value of the 'time' field.
   */
  public java.time.Instant getTime() {
    return time;
  }


  /**
   * Sets the value of the 'time' field.
   * @param value the value to set.
   */
  public void setTime(java.time.Instant value) {
    this.time = value.truncatedTo(java.time.temporal.ChronoUnit.MILLIS);
  }

  /**
   * Gets the value of the 'origin' field.
   * @return The value of the 'origin' field.
   */
  public java.lang.CharSequence getOrigin() {
    return origin;
  }


  /**
   * Sets the value of the 'origin' field.
   * @param value the value to set.
   */
  public void setOrigin(java.lang.CharSequence value) {
    this.origin = value;
  }

  /**
   * Gets the value of the 'brand_id' field.
   * @return The value of the 'brand_id' field.
   */
  public java.lang.CharSequence getBrandId() {
    return brand_id;
  }


  /**
   * Sets the value of the 'brand_id' field.
   * @param value the value to set.
   */
  public void setBrandId(java.lang.CharSequence value) {
    this.brand_id = value;
  }

  /**
   * Gets the value of the 'category_id' field.
   * @return The value of the 'category_id' field.
   */
  public java.lang.CharSequence getCategoryId() {
    return category_id;
  }


  /**
   * Sets the value of the 'category_id' field.
   * @param value the value to set.
   */
  public void setCategoryId(java.lang.CharSequence value) {
    this.category_id = value;
  }

  /**
   * Creates a new AggregatesItem RecordBuilder.
   * @return A new AggregatesItem RecordBuilder
   */
  public static allezone_analytics.AggregatesItem.Builder newBuilder() {
    return new allezone_analytics.AggregatesItem.Builder();
  }

  /**
   * Creates a new AggregatesItem RecordBuilder by copying an existing Builder.
   * @param other The existing builder to copy.
   * @return A new AggregatesItem RecordBuilder
   */
  public static allezone_analytics.AggregatesItem.Builder newBuilder(allezone_analytics.AggregatesItem.Builder other) {
    if (other == null) {
      return new allezone_analytics.AggregatesItem.Builder();
    } else {
      return new allezone_analytics.AggregatesItem.Builder(other);
    }
  }

  /**
   * Creates a new AggregatesItem RecordBuilder by copying an existing AggregatesItem instance.
   * @param other The existing instance to copy.
   * @return A new AggregatesItem RecordBuilder
   */
  public static allezone_analytics.AggregatesItem.Builder newBuilder(allezone_analytics.AggregatesItem other) {
    if (other == null) {
      return new allezone_analytics.AggregatesItem.Builder();
    } else {
      return new allezone_analytics.AggregatesItem.Builder(other);
    }
  }

  /**
   * RecordBuilder for AggregatesItem instances.
   */
  @org.apache.avro.specific.AvroGenerated
  public static class Builder extends org.apache.avro.specific.SpecificRecordBuilderBase<AggregatesItem>
    implements org.apache.avro.data.RecordBuilder<AggregatesItem> {

    private java.time.Instant time;
    private java.lang.CharSequence origin;
    private java.lang.CharSequence brand_id;
    private java.lang.CharSequence category_id;

    /** Creates a new Builder */
    private Builder() {
      super(SCHEMA$, MODEL$);
    }

    /**
     * Creates a Builder by copying an existing Builder.
     * @param other The existing Builder to copy.
     */
    private Builder(allezone_analytics.AggregatesItem.Builder other) {
      super(other);
      if (isValidValue(fields()[0], other.time)) {
        this.time = data().deepCopy(fields()[0].schema(), other.time);
        fieldSetFlags()[0] = other.fieldSetFlags()[0];
      }
      if (isValidValue(fields()[1], other.origin)) {
        this.origin = data().deepCopy(fields()[1].schema(), other.origin);
        fieldSetFlags()[1] = other.fieldSetFlags()[1];
      }
      if (isValidValue(fields()[2], other.brand_id)) {
        this.brand_id = data().deepCopy(fields()[2].schema(), other.brand_id);
        fieldSetFlags()[2] = other.fieldSetFlags()[2];
      }
      if (isValidValue(fields()[3], other.category_id)) {
        this.category_id = data().deepCopy(fields()[3].schema(), other.category_id);
        fieldSetFlags()[3] = other.fieldSetFlags()[3];
      }
    }

    /**
     * Creates a Builder by copying an existing AggregatesItem instance
     * @param other The existing instance to copy.
     */
    private Builder(allezone_analytics.AggregatesItem other) {
      super(SCHEMA$, MODEL$);
      if (isValidValue(fields()[0], other.time)) {
        this.time = data().deepCopy(fields()[0].schema(), other.time);
        fieldSetFlags()[0] = true;
      }
      if (isValidValue(fields()[1], other.origin)) {
        this.origin = data().deepCopy(fields()[1].schema(), other.origin);
        fieldSetFlags()[1] = true;
      }
      if (isValidValue(fields()[2], other.brand_id)) {
        this.brand_id = data().deepCopy(fields()[2].schema(), other.brand_id);
        fieldSetFlags()[2] = true;
      }
      if (isValidValue(fields()[3], other.category_id)) {
        this.category_id = data().deepCopy(fields()[3].schema(), other.category_id);
        fieldSetFlags()[3] = true;
      }
    }

    /**
      * Gets the value of the 'time' field.
      * @return The value.
      */
    public java.time.Instant getTime() {
      return time;
    }


    /**
      * Sets the value of the 'time' field.
      * @param value The value of 'time'.
      * @return This builder.
      */
    public allezone_analytics.AggregatesItem.Builder setTime(java.time.Instant value) {
      validate(fields()[0], value);
      this.time = value.truncatedTo(java.time.temporal.ChronoUnit.MILLIS);
      fieldSetFlags()[0] = true;
      return this;
    }

    /**
      * Checks whether the 'time' field has been set.
      * @return True if the 'time' field has been set, false otherwise.
      */
    public boolean hasTime() {
      return fieldSetFlags()[0];
    }


    /**
      * Clears the value of the 'time' field.
      * @return This builder.
      */
    public allezone_analytics.AggregatesItem.Builder clearTime() {
      fieldSetFlags()[0] = false;
      return this;
    }

    /**
      * Gets the value of the 'origin' field.
      * @return The value.
      */
    public java.lang.CharSequence getOrigin() {
      return origin;
    }


    /**
      * Sets the value of the 'origin' field.
      * @param value The value of 'origin'.
      * @return This builder.
      */
    public allezone_analytics.AggregatesItem.Builder setOrigin(java.lang.CharSequence value) {
      validate(fields()[1], value);
      this.origin = value;
      fieldSetFlags()[1] = true;
      return this;
    }

    /**
      * Checks whether the 'origin' field has been set.
      * @return True if the 'origin' field has been set, false otherwise.
      */
    public boolean hasOrigin() {
      return fieldSetFlags()[1];
    }


    /**
      * Clears the value of the 'origin' field.
      * @return This builder.
      */
    public allezone_analytics.AggregatesItem.Builder clearOrigin() {
      origin = null;
      fieldSetFlags()[1] = false;
      return this;
    }

    /**
      * Gets the value of the 'brand_id' field.
      * @return The value.
      */
    public java.lang.CharSequence getBrandId() {
      return brand_id;
    }


    /**
      * Sets the value of the 'brand_id' field.
      * @param value The value of 'brand_id'.
      * @return This builder.
      */
    public allezone_analytics.AggregatesItem.Builder setBrandId(java.lang.CharSequence value) {
      validate(fields()[2], value);
      this.brand_id = value;
      fieldSetFlags()[2] = true;
      return this;
    }

    /**
      * Checks whether the 'brand_id' field has been set.
      * @return True if the 'brand_id' field has been set, false otherwise.
      */
    public boolean hasBrandId() {
      return fieldSetFlags()[2];
    }


    /**
      * Clears the value of the 'brand_id' field.
      * @return This builder.
      */
    public allezone_analytics.AggregatesItem.Builder clearBrandId() {
      brand_id = null;
      fieldSetFlags()[2] = false;
      return this;
    }

    /**
      * Gets the value of the 'category_id' field.
      * @return The value.
      */
    public java.lang.CharSequence getCategoryId() {
      return category_id;
    }


    /**
      * Sets the value of the 'category_id' field.
      * @param value The value of 'category_id'.
      * @return This builder.
      */
    public allezone_analytics.AggregatesItem.Builder setCategoryId(java.lang.CharSequence value) {
      validate(fields()[3], value);
      this.category_id = value;
      fieldSetFlags()[3] = true;
      return this;
    }

    /**
      * Checks whether the 'category_id' field has been set.
      * @return True if the 'category_id' field has been set, false otherwise.
      */
    public boolean hasCategoryId() {
      return fieldSetFlags()[3];
    }


    /**
      * Clears the value of the 'category_id' field.
      * @return This builder.
      */
    public allezone_analytics.AggregatesItem.Builder clearCategoryId() {
      category_id = null;
      fieldSetFlags()[3] = false;
      return this;
    }

    @Override
    @SuppressWarnings("unchecked")
    public AggregatesItem build() {
      try {
        AggregatesItem record = new AggregatesItem();
        record.time = fieldSetFlags()[0] ? this.time : (java.time.Instant) defaultValue(fields()[0]);
        record.origin = fieldSetFlags()[1] ? this.origin : (java.lang.CharSequence) defaultValue(fields()[1]);
        record.brand_id = fieldSetFlags()[2] ? this.brand_id : (java.lang.CharSequence) defaultValue(fields()[2]);
        record.category_id = fieldSetFlags()[3] ? this.category_id : (java.lang.CharSequence) defaultValue(fields()[3]);
        return record;
      } catch (org.apache.avro.AvroMissingFieldException e) {
        throw e;
      } catch (java.lang.Exception e) {
        throw new org.apache.avro.AvroRuntimeException(e);
      }
    }
  }

  @SuppressWarnings("unchecked")
  private static final org.apache.avro.io.DatumWriter<AggregatesItem>
    WRITER$ = (org.apache.avro.io.DatumWriter<AggregatesItem>)MODEL$.createDatumWriter(SCHEMA$);

  @Override public void writeExternal(java.io.ObjectOutput out)
    throws java.io.IOException {
    WRITER$.write(this, SpecificData.getEncoder(out));
  }

  @SuppressWarnings("unchecked")
  private static final org.apache.avro.io.DatumReader<AggregatesItem>
    READER$ = (org.apache.avro.io.DatumReader<AggregatesItem>)MODEL$.createDatumReader(SCHEMA$);

  @Override public void readExternal(java.io.ObjectInput in)
    throws java.io.IOException {
    READER$.read(this, SpecificData.getDecoder(in));
  }

}









