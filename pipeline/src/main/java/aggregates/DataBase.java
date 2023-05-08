package aggregates;

import java.sql.*;
import java.util.List;
import java.util.Properties;

import allezone_analytics.AggregatesItem;
import io.github.cdimascio.dotenv.Dotenv;

public final class DataBase {
	private final Connection conn;

	public DataBase() throws SQLException {
		Dotenv dotenv = Dotenv.load();

		Properties props = buildProps(dotenv);
		String url = buildUrl(dotenv);

		conn = DriverManager.getConnection(url, props);
	}

	public void batchInsert(List<AggregatesDBItem> dbItems) {
		try {
			PreparedStatement stmt = conn.prepareStatement("INSERT INTO aggregates (time, action, origin, brand_id, category_id, count, sum) VALUES (?, ?, ?, ?, ?, ?, ?)");
			for(AggregatesDBItem item: dbItems) {
				stmt.setTimestamp(1, item.time);
				stmt.setString(2, item.action);
				stmt.setString(3, item.origin);
				stmt.setString(4, item.brand_id);
				stmt.setString(5, item.category_id);
				stmt.setLong(6, item.count);
				stmt.setLong(7, item.sum);
				stmt.addBatch();
			}
			stmt.executeBatch();
		} catch (SQLException e) {
			e.printStackTrace();
		}
	}

	static private String buildUrl(Dotenv dotenv) {
		StringBuilder url = new StringBuilder();
		url.append("jdbc:postgresql://");
		url.append(dotenv.get("DB_HOST"));
		url.append(":");
		url.append(dotenv.get("DB_PORT"));
		url.append("/");
		url.append(dotenv.get("DB_NAME"));
		return url.toString();
	}

	static private Properties buildProps(Dotenv dotenv) {
		Properties props = new Properties();
		props.setProperty("user", dotenv.get("DB_USER"));
		props.setProperty("password", dotenv.get("DB_PASSWORD"));
		props.setProperty("ssl", "false");
		return props;
	}

	public static class AggregatesDBItem {
		private final Timestamp time;
		private final String action;
		private final String origin;
		private final String brand_id;
		private final String category_id;
		private final Long count;
		private final Long sum;

		public AggregatesDBItem(AggregatesItem item, Long count, Long sum) {
			this.time = Timestamp.from(item.getTime());
			this.action = item.getAction().toString();
			this.origin = item.getOrigin().toString();
			this.brand_id = item.getBrandId().toString();
			this.category_id = item.getCategoryId().toString();
			this.count = count;
			this.sum = sum;
		}
	}
}
