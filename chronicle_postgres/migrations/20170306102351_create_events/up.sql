CREATE TABLE events (
  global_sequence_number BIGSERIAL NOT NULL,
  source_id UUID NOT NULL,
  source_sequence_number BIGINT NOT NULL,
  data BYTEA NOT NULL,
  PRIMARY KEY(source_id, source_sequence_number),
);
