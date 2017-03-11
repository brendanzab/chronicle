CREATE TABLE events (
  offset BIGSERIAL NOT NULL,
  source_id UUID NOT NULL,
  sequence_number BIGINT NOT NULL,
  data BYTEA NOT NULL,
  created_at timestamp not null,
  PRIMARY KEY(source_id, sequence_number),
);
