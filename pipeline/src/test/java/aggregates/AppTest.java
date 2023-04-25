package aggregates;

import static org.junit.Assert.assertTrue;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.PrintStream;
import java.time.Instant;
import java.util.Arrays;
import java.util.List;

import org.apache.avro.io.BinaryDecoder;
import org.apache.avro.io.BinaryEncoder;
import org.apache.avro.io.DatumReader;
import org.apache.avro.io.DatumWriter;
import org.apache.avro.io.Decoder;
import org.apache.avro.io.DecoderFactory;
import org.apache.avro.io.EncoderFactory;
import org.apache.avro.message.BinaryMessageDecoder;
import org.apache.avro.specific.SpecificDatumReader;
import org.apache.avro.specific.SpecificDatumWriter;
import org.junit.Before;
import org.junit.Test;

import allezone_analytics.AggregatesItem;
import allezone_analytics.AggregatesPrice;

/**
 * Unit test for simple App.
 */
public class AppTest 

{
    private final ByteArrayOutputStream outputStreamCaptor = new ByteArrayOutputStream();

    @Before
    public void setUp() {
        System.setOut(new PrintStream(outputStreamCaptor));
    }
    /**
     * Rigorous Test :-)
     */
    @Test
    public void testSchemaAggregatesItem()
    {
        // byte[] aggregatesItemBytes = hexStringToByteArray("4f626a0104166176726f2e736368656d61d8047b2274797065223a227265636f7264222c226e616d657370616365223a22616c6c657a6f6e655f616e616c7974696373222c226e616d65223a22416767726567617465734974656d222c226669656c6473223a5b7b226e616d65223a2274696d65222c2274797065223a7b2274797065223a226c6f6e67222c226c6f676963616c54797065223a2274696d657374616d702d6d696c6c6973227d7d2c7b226e616d65223a22616374696f6e222c2274797065223a22737472696e67227d2c7b226e616d65223a226f726967696e222c2274797065223a22737472696e67227d2c7b226e616d65223a226272616e645f6964222c2274797065223a22737472696e67227d2c7b226e616d65223a2263617465676f72795f6964222c2274797065223a22737472696e67227d5d7d146176726f2e636f646563086e756c6c0032d006af34c413f32f5f34709c6ea36d022ea0ec8df8f661085649455706616263066162630661626332d006af34c413f32f5f34709c6ea36d");
        
        List<Integer> intList = Arrays.asList(250, 249, 171, 252, 246, 97, 8, 86, 73, 69, 87, 6, 97, 98, 99, 6, 97, 98, 99, 6, 97, 98, 99);
        byte[] aggregatesItemBytes = new byte[intList.size()];

        for (int i = 0; i < intList.size(); i++) {
            aggregatesItemBytes[i] = intList.get(i).byteValue();
        }
        DatumReader<AggregatesItem> reader = new SpecificDatumReader<AggregatesItem>(AggregatesItem.SCHEMA$, AggregatesItem.SCHEMA$);
        BinaryDecoder decoder = DecoderFactory.get().binaryDecoder(aggregatesItemBytes, null);
        try {
            reader.read(null, decoder);
        } catch (IOException e) {
        e.printStackTrace();
        }
        assertTrue( true );
    }

    @Test
    public void trySerialize()
    {
        AggregatesItem aggregatesItem = new AggregatesItem();
        aggregatesItem.setAction("abc");
        aggregatesItem.setBrandId("abc");
        aggregatesItem.setCategoryId("abc");
        aggregatesItem.setOrigin("abc");
        aggregatesItem.setTime(Instant.now());
        DatumWriter<AggregatesItem> writer = new SpecificDatumWriter<AggregatesItem>(AggregatesItem.SCHEMA$);
        ByteArrayOutputStream out = new ByteArrayOutputStream();
        BinaryEncoder encoder = EncoderFactory.get().binaryEncoder(out, null);
        try {
            writer.write(aggregatesItem, encoder);
            encoder.flush();
            
            System.out.println(out.toByteArray());

            DatumReader<AggregatesItem> reader = new SpecificDatumReader<AggregatesItem>(AggregatesItem.SCHEMA$, AggregatesItem.SCHEMA$);
            BinaryDecoder decoder = DecoderFactory.get().binaryDecoder(out.toByteArray(), null);
            try {
                reader.read(null, decoder);
            } catch (IOException e) {
            e.printStackTrace();
            }
        } catch (IOException e) {
            e.printStackTrace();
        }
        assertTrue( true );
    }

    // @Test
    // public void testSchemaAggregatesPrice()
    // {        
    //     List<Integer> intList = Arrays.asList(246, 1);
    //     byte[] aggregatesPriceBytes = new byte[intList.size()];

    //     for (int i = 0; i < intList.size(); i++) {
    //         aggregatesPriceBytes[i] = intList.get(i).byteValue();
    //     }
        
    //     DatumReader<AggregatesPrice> reader = new SpecificDatumReader<AggregatesPrice>(AggregatesPrice.SCHEMA$);
    //     Decoder decoder = DecoderFactory.get().binaryDecoder(aggregatesPriceBytes, null);
    //     try {
    //         reader.read(null, decoder);
    //     } catch (IOException e) {
    //     e.printStackTrace();
    //     }
    //     assertTrue( true );
    // }

    public static byte[] hexStringToByteArray(String s) {
        int len = s.length();
        byte[] data = new byte[len / 2];
        for (int i = 0; i < len; i += 2) {
            data[i / 2] = (byte) ((Character.digit(s.charAt(i), 16) << 4)
                                 + Character.digit(s.charAt(i+1), 16));
        }
        return data;
    }
}
