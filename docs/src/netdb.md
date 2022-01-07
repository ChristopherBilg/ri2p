# NetDB

The i2p networking database (NetDB) is a specialized distributed database, also known as a [distributed hash table](https://en.wikipedia.org/wiki/Distributed_hash_table) (DHT).

All data that lives inside of the NetDB must be cryptographically signed and all data that is retrieved from the NetDB must be cryptographically verified for authenticity. This data also has information regarding the timeframe that it is healthy to be inside of the database.

The NetDb is distributed using a "floodfill". A portion of all i2p routers, aptly named "floodfill routers", maintain the NetDB DHT.
