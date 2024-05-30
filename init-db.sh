psql -d postgres -c 'CREATE EXTENSION IF NOT EXISTS mumak;'

psql -d postgres <<EOF
-- Table for storing CBOR blocks
CREATE TABLE blocks (
    slot INTEGER NOT NULL,
    cbor BYTEA
);

-- Index for the blocks table
CREATE INDEX idx_blocks_slot ON blocks(slot);

-- Table for storing CBOR transactions
CREATE TABLE txs (
    slot INTEGER NOT NULL,
    cbor BYTEA
);

-- Index for the txs table
CREATE INDEX idx_txs_slot ON txs(slot);
EOF

psql -d postgres -c "\COPY blocks FROM '/data/latest_10k_blocks.csv' WITH (FORMAT csv, HEADER true);"
psql -d postgres -c "\COPY txs FROM '/data/latest_10k_transactions.csv' WITH (FORMAT csv, HEADER true);"

psql -d postgres -c 'CREATE TABLE utxos (hash BYTEA, output_index INTEGER, slot INTEGER, era INTEGER, body BYTEA);'
psql -d postgres -c "INSERT INTO utxos (hash, output_index, slot, era, body) VALUES(E'\\\\xa30',0,0, 5, E'\\\\xa300581d7161b3802ce748ed1fdaad2d6c744b19f104285f7d318172a5d4f06a4e01821a001ab364a1581cc27600f3aff3d94043464a33786429b78e6ab9df5e1d23b774acb34ca144434e43541b0000001369fb413e028201d8185899d8799f9fd8799f1a9a7ec800d8799f0118c8ffffd8799f1b00000001cf7c5800d8799f0518c8ffffd8799f1b000000039ef8b000d8799f0f18c8ffffd8799f1b000000073df16000d8799f182818c8ffffff581cc27600f3aff3d94043464a33786429b78e6ab9df5e1d23b774acb34c44434e4354d8799f581c63ea9fddc8d7d0dc87c7eef00df01c6e01c028b461c1b62c82aa9e37ff00ff');"
psql -d postgres -c "INSERT INTO utxos (hash, output_index, slot, era, body) VALUES ('\\\\xa30',0,0, 5, E'\\\\xa300581d70f1a11479824f2bbb70fa53e5cdf93bceb16f6a71b0dae6c4562212eb01821a00137770a1581cf1a11479824f2bbb70fa53e5cdf93bceb16f6a71b0dae6c4562212eba145534849503001028201d8185832d8799f0a1629581cf1a11479824f2bbb70fa53e5cdf93bceb16f6a71b0dae6c4562212eb4553484950304650494c4f5430ff');"
psql -d postgres -c "INSERT INTO utxos (hash, output_index, slot, era, body) VALUES ('\\\\xa30',0,0, 5, E'\\\\xa300581d7051745ce960e0cbecd83adfaa8d695ce6602220e12bc9417dcf8545fc01821a0012be3ea1581c0298aa99f95e2fe0a0132a6bb794261fb7e7b0d988215da2f2de2005a146746f6b656e4101028201d8185826d8799f18280736581c914a4a8ea99d4190a131ff44a5864b4470f02a453c67ce916ce2ed68ff');"
psql -d postgres -c "INSERT INTO utxos (hash, output_index, slot, era, body) VALUES ('\\\\xa30',0,0, 5, E'\\\\xa300581d7082d506586b6a6c1b6196757e734c1a11dd633a5c21797603118f608801821a00129774a1581c0298aa99f95e2fe0a0132a6bb794261fb7e7b0d988215da2f2de2005a146746f6b656e4101028201d8185823d8799f01581cf1a11479824f2bbb70fa53e5cdf93bceb16f6a71b0dae6c4562212ebff');"