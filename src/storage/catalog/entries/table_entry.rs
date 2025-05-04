pub struct TableEntry {
    table_name: String,
    oid: u16,
    columns: u16,
    col_map_pg_num: u32,
    ref_map_pg_num: u32,
    no_of_cols_in_primary_key: u8,
}
