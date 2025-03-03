## Tests for vchord_bm25

We use [sqllogictest-rs](https://github.com/risinglightdb/sqllogictest-rs) to test the SQL queries.

To run all tests, use the following command:
```shell
sqllogictest './tests/**/*.slt'
```

Each time you modify the source code, you can run the following command to clean up the test data and reload the extension:
```shell
psql -f ./tests/init.sql
```

## Note

For each test, it'll rollback the transaction after the test, so you don't need to worry about the side effects of the tests. If you want to keep the data after the test for debugging, you can add the following line to the end of the test:
```sql
statement ok
COMMIT;
```
