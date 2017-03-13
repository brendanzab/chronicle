CREATE TABLE events (
  offset BIGSERIAL NOT NULL,
  source_id UUID NOT NULL,
  sequence_number BIGINT NOT NULL,
  payload BYTEA NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT now(),
  PRIMARY KEY(source_id, sequence_number),
);
