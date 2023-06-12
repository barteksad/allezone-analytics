CREATE DATABASE allezone_analytics;
GRANT ALL PRIVILEGES ON DATABASE allezone_analytics TO postgres;

\connect allezone_analytics

CREATE TABLE aggregates
(
	time		TIMESTAMP NOT NULL, 
	action 		text NOT NULL,
	origin		text NOT NULL,
	brand_id	text NOT NULL,
	category_id text NOT NULL,
	count 		BIGINT NOT NULL,
	sum 		BIGINT NOT NULL,
	
	PRIMARY KEY (time, action, origin, brand_id, category_id)
);

CREATE INDEX count_lookup1 ON aggregates (time, action);

-- Add triggers which removes items older than 24h than currently inserted one
CREATE OR REPLACE FUNCTION remove_older_items() RETURNS TRIGGER AS $$
BEGIN
	DELETE FROM aggregates WHERE time < (NEW.time - INTERVAL '24 hours');
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER remove_older_items_trigger
AFTER INSERT OR UPDATE ON aggregates
EXECUTE PROCEDURE remove_older_items();


