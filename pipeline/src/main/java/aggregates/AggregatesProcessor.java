package aggregates;

import java.sql.SQLException;
import java.time.Duration;
import java.util.ArrayList;
import java.util.List;

import org.apache.commons.lang3.tuple.Triple;
import org.apache.kafka.streams.KeyValue;
import org.apache.kafka.streams.processor.PunctuationType;
import org.apache.kafka.streams.processor.api.Processor;
import org.apache.kafka.streams.processor.api.ProcessorContext;
import org.apache.kafka.streams.processor.api.Record;
import org.apache.kafka.streams.state.KeyValueIterator;
import org.apache.kafka.streams.state.KeyValueStore;

import aggregates.DataBase.AggregatesDBItem;
import allezone_analytics.AggregatesItem;
import allezone_analytics.AggregatesPrice;

public class AggregatesProcessor implements Processor<AggregatesItem, AggregatesPrice, AggregatesItem, Long> {
	private KeyValueStore<AggregatesItem, Long> countPrice;
	private KeyValueStore<AggregatesItem, Long> sumPrice;
	private DataBase db;

	@Override
	public void init(ProcessorContext<AggregatesItem, Long> context) {
		countPrice = context.getStateStore("aggregates-price-count");
		sumPrice = context.getStateStore("aggregates-price-sum");

		try {
			db = new DataBase();
		} catch (SQLException e) {
			e.printStackTrace();
			System.exit(222);
		}

		context.schedule(Duration.ofSeconds(10), PunctuationType.STREAM_TIME, timestamp -> {
			List<Triple<AggregatesItem, Long, Long>> processedAggregates = new ArrayList<>();
			List<AggregatesDBItem> dbItems = new ArrayList<>();
			try (final KeyValueIterator<AggregatesItem, Long> iter = countPrice.all()) {
				while (iter.hasNext()) {
					final KeyValue<AggregatesItem, Long> entry = iter.next();
					AggregatesItem userId = entry.key;
					Long count = entry.value;
					Long sum = sumPrice.get(userId);
					processedAggregates.add(Triple.of(userId, count, sum));
					dbItems.add(new AggregatesDBItem(userId, count, sum));
				}
			}
			System.out.println("Processed " + processedAggregates.size() + " items");

			if (db.batchInsert(dbItems)) {
				System.out.println("Inserted " + dbItems.size() + " items");
				for (Triple<AggregatesItem, Long, Long> item : processedAggregates) {
					countPrice.delete(item.getLeft());
					sumPrice.delete(item.getLeft());
				}
			} else {
				System.out.println("Failed to insert " + dbItems.size() + " items");
			}

		});
	}

	@Override
	public void process(Record<AggregatesItem, AggregatesPrice> record) {
		AggregatesItem item = record.key();
		AggregatesPrice price = record.value();

		Long count = countPrice.get(item);
		if (count == null) {
			count = 0L;
		}
		countPrice.put(item, count + 1);

		Long sum = sumPrice.get(item);
		if (sum == null) {
			sum = 0L;
		}
		sumPrice.put(item, sum + price.getPrice());
	}
}
