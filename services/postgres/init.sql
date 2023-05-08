CREATE DATABASE allezone_analytics;
GRANT ALL PRIVILEGES ON DATABASE allezone_analytics TO postgres;

\connect allezone_analytics

CREATE TABLE aggregates
(
	time		TIMESTAMP NOT NULL, 
	action 		VARCHAR(255) NOT NULL,
	origin		VARCHAR(255) NOT NULL,
	brand_id	VARCHAR(255) NOT NULL,
	category_id VARCHAR(255) NOT NULL,
	count 		BIGINT NOT NULL,
	sum 		BIGINT NOT NULL,
	
	PRIMARY KEY (time, action, origin, brand_id, category_id)
);

CREATE INDEX count_lookup1 ON aggregates (time, action);