from cassandra.cluster import Cluster
from uuid import uuid4
from datetime import datetime, timedelta
import random

# -------------------
# CONFIG
# -------------------
TOTAL_ROWS = 20000
ACCOUNTS = [1,2,3,4,5]

# -------------------
# CONNECT
# -------------------
cluster = Cluster(["127.0.0.1"])
session = cluster.connect("bank")

print("Seeding dummy banking transactions...")

query = session.prepare("""
INSERT INTO transactions
(account_id, tx_time, tx_id, amount)
VALUES (?, ?, ?, ?)
""")

for i in range(TOTAL_ROWS):

    account = random.choice(ACCOUNTS)

    # random timestamp in last 30 days
    tx_time = datetime.utcnow() - timedelta(
        minutes=random.randint(0, 60*24*30)
    )

    session.execute(query, (
        account,
        tx_time,
        uuid4(),
        round(random.uniform(5,500),2)
    ))

    if i > 0 and i % 5000 == 0:
        print(f"Inserted {i} rows")

print("Dummy data ready.")