# Limitation

## object cache may not follow transaction isolation level

`pg_tokenizer` will cache `text analyzer`, `model` and `tokenizer` object in memory for each connection. The cache will always be updated when calling `create_...`, `drop_...` functions. And it doesn't follow transaction isolation level.

The cache may not be cleared when you rollback a transaction. You may need to call `drop_...` functions manually or reconnect to the database to clear the cache.
