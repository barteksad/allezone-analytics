package aggregates;

import java.sql.SQLException;
import java.time.Duration;
import java.time.Instant;
import java.time.temporal.ChronoUnit;
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
	private ProcessorContext<AggregatesItem, Long> processorcContext;
	private DataBase db;

	@Override
	public void init(ProcessorContext<AggregatesItem, Long> context) {
		countPrice = context.getStateStore("aggregates-price-count");
		sumPrice = context.getStateStore("aggregates-price-sum");
		processorcContext = context;

		try {
			db = new DataBase();
		} catch (SQLException e) {
			e.printStackTrace();
			System.exit(222);
		}

		context.schedule(Duration.ofSeconds(10), PunctuationType.WALL_CLOCK_TIME, timestamp -> {
			List<Triple<AggregatesItem, Long, Long>> processedAggregates = new ArrayList<>();
			try (final KeyValueIterator<AggregatesItem, Long> iter = countPrice.all()) {
				while (iter.hasNext()) {
					final KeyValue<AggregatesItem, Long> entry = iter.next();
					AggregatesItem userId = entry.key;
					Long count = entry.value;
					Long sum = sumPrice.get(userId);
					processedAggregates.add(Triple.of(userId, count, sum));
				}
			}
			System.out.println("Processed " + processedAggregates.size() + " items");

			List<AggregatesDBItem> dbItems = new ArrayList<>();
			for (Triple<AggregatesItem, Long, Long> item : processedAggregates) {
					dbItems.add(new AggregatesDBItem(item.getLeft(), item.getMiddle(), item.getRight()));
					countPrice.delete(item.getLeft());
					sumPrice.delete(item.getLeft());
			}

			System.out.println("Inserting " + dbItems.size() + " items");
			db.batchInsert(dbItems);

			countPrice.flush();
			sumPrice.flush();

			processorcContext.commit();
		});
	}

	@Override
	public void process(Record<AggregatesItem, AggregatesPrice> record) {
		AggregatesItem item = record.key();
		AggregatesPrice price = record.value();

		item.setTime(item.getTime().truncatedTo(ChronoUnit.MINUTES));

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
