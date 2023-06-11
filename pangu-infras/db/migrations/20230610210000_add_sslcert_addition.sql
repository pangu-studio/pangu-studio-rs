ALTER TABLE ssl_certificates ADD COLUMN sn TEXT NOT NULL;
ALTER TABLE ssl_certificates ADD COLUMN addition TEXT DEFAULT '{}';
CREATE INDEX IF NOT EXISTS idx_ssl_certificates_sn ON ssl_certificates (sn);