CREATE KEYSPACE bank
WITH replication = {'class':'SimpleStrategy','replication_factor':1};

USE bank;

CREATE TABLE transactions (
    account_id int,
    tx_time timestamp,
    tx_id uuid,
    amount double,
    PRIMARY KEY (account_id, tx_time, tx_id)
) WITH gc_grace_seconds = 3600;