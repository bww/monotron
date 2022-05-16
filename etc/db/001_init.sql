
CREATE TABLE IF NOT EXISTS mn_account (
  id         BIGSERIAL PRIMARY KEY,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS mn_api_key (
  id         BIGSERIAL PRIMARY KEY,
  key        VARCHAR(256) NOT NULL UNIQUE,
  secret     VARCHAR(1024) NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS mn_account_r_api_key (
  account_id BIGINT NOT NULL REFERENCES mn_account (id),
  api_key_id BIGINT NOT NULL REFERENCES mn_api_key (id),
  scopes     VARCHAR(64)[] NOT NULL,
  PRIMARY KEY (account_id, api_key_id)
);

INSERT INTO mn_account (id) VALUES (0)
  ON CONFLICT (id) DO NOTHING;

CREATE TABLE IF NOT EXISTS mn_entry (
  key        VARCHAR(256) NOT NULL,
  creator_id BIGINT NOT NULL REFERENCES mn_account (id),
  token      VARCHAR(256), -- nullable
  value      BIGINT NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
  PRIMARY KEY (key, creator_id)
);

CREATE TABLE IF NOT EXISTS mn_entry_version (
  key        VARCHAR(256) NOT NULL,
  creator_id BIGINT NOT NULL REFERENCES mn_account (id),
  token      VARCHAR(256), -- not nullable in version history
  value      BIGINT NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
  PRIMARY KEY (key, creator_id, token)
);
