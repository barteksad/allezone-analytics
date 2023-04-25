package aggregates;

import java.time.Duration;
import java.time.Instant;
import java.time.temporal.ChronoUnit;
import java.util.ArrayList;
import java.util.Calendar;
import java.util.List;

import org.apache.commons.lang3.time.DateUtils;
import org.apache.kafka.streams.KeyValue;
import org.apache.kafka.streams.processor.PunctuationType;
import org.apache.kafka.streams.processor.api.Processor;
import org.apache.kafka.streams.processor.api.ProcessorContext;
import org.apache.kafka.streams.processor.api.Record;
import org.apache.kafka.streams.state.KeyValueIterator;
import org.apache.kafka.streams.state.KeyValueStore;

import allezone_analytics.AggregatesItem;
import allezone_analytics.AggregatesPrice;

public class AggregatesProcessor implements Processor<AggregatesItem, AggregatesPrice, AggregatesItem, Long> {
  private KeyValueStore<AggregatesItem, Long> countPrice;
  private KeyValueStore<AggregatesItem, Long> sumPrice;
  private ProcessorContext<AggregatesItem, Long> processorcContext;

  @Override
  public void init(ProcessorContext<AggregatesItem, Long> context) {
    countPrice = context.getStateStore("aggregates-price-count");
    sumPrice = context.getStateStore("aggregates-price-sum");
    processorcContext = context;

    context.schedule(Duration.ofSeconds(1), PunctuationType.WALL_CLOCK_TIME, timestamp -> {
      List<AggregatesItem> aggregates = new ArrayList<>();
      try (final KeyValueIterator<AggregatesItem, Long> iter = countPrice.all()) {
        while (iter.hasNext()) {
          final KeyValue<AggregatesItem, Long> entry = iter.next();
          AggregatesItem userId = entry.key;
          Long count = entry.value;
          Long sum = sumPrice.get(userId);
          aggregates.add(userId);
          // profiles.add(new DatabaseMock.UserProfile(userId, count, sum));
        }
      }
      System.out.println("Processed " + aggregates.size() + " items");
      // database.batchUpdate(profiles);

      for(AggregatesItem item: aggregates) {
        countPrice.delete(item);
        sumPrice.delete(item);
      }

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
