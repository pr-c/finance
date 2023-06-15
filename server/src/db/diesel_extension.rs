use diesel::prelude::*;

sql_function!(
    fn last_insert_id() -> Unsigned<Integer>
);
