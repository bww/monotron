
ALTER TABLE mn_entry_version ALTER COLUMN token SET NOT NULL;

CREATE TABLE mn_entry_version_attr (
  key         VARCHAR(256) NOT NULL,
  creator_id  BIGINT NOT NULL,
  token       VARCHAR(256) NOT NULL,
  name        VARCHAR(256) NOT NULL,
  value       TEXT,
  created_at  TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
  updated_at  TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
  FOREIGN KEY (key, creator_id, token) REFERENCES mn_entry_version (key, creator_id, token),
  PRIMARY KEY (key, creator_id, token, name)
);
