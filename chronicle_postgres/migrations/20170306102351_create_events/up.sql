CREATE TABLE events (
  offset BIGSERIAL NOT NULL,
  source_id UUID NOT NULL,
  sequence_number BIGINT NOT NULL,
  payload BYTEA NOT NULL,
  created_at timestamp not null,
  PRIMARY KEY(source_id, sequence_number),
);
