# HintedID

Adds `hintedid` type to PostgreSQL and the `generate_hinted_id(text)` function.

Hinted IDs are a 20 character ID inspired by Stripe's ID format:

 - variable length text prefix
 - `_`
 - 20 character base32 XID using [xid](https://github.com/kazk/xid-rs)

```sql
SELECT generate_hinted_id('user');
   generate_hinted_id    
-------------------------
 user_c3ibeg2ciaeol4vj3p0g
(1 row)

SELECT generate_hinted_id('user');
   generate_hinted_id    
-------------------------
 user_c3ibeg2ciaeol4vj3p0g
(1 row)
```

### Contributing

Must have `cargo-pgx` to have `cargo pgx init` run so the build scripts work.
