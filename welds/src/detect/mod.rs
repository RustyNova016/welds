use crate::errors::Result;
use crate::table::{ColumnDef, RelationDef, TableDef, TableIdent};
use sqlx::database::HasArguments;
use sqlx::{IntoArguments, Row};
use std::collections::HashMap;

mod table_scan;
use table_scan::{FkScanRow, FkScanTableCol, TableScan, TableScanRow};

/// Returns a list of all user defined tables in the database
/// requires feature `detect`
pub async fn find_tables<'i, 's, 'e, 'ee, 'args, DB, E>(exec: &'ee E) -> Result<Vec<TableDef>>
where
    &'ee E: sqlx::Executor<'e, Database = DB>,
    <DB as HasArguments<'args>>::Arguments: IntoArguments<'args, DB>,
    DB: sqlx::Database + TableScan,
    String: sqlx::Type<DB> + for<'r> sqlx::Decode<'r, DB>,
    i32: sqlx::Type<DB> + for<'r> sqlx::Decode<'r, DB>,
    Option<String>: sqlx::Type<DB> + for<'r> sqlx::Decode<'r, DB>,
    usize: sqlx::ColumnIndex<<DB as sqlx::Database>::Row>,
{
    let sql = DB::table_scan_sql();
    let q = sqlx::query(sql);
    let rows: Vec<_> = q
        .fetch_all(exec)
        .await?
        .iter()
        .map(|r| TableScanRow {
            schema: r.get(0),
            table_name: r.get(1),
            ty: r.get(2),
            column_name: r.get(3),
            column_type: r.get(4),
            is_nullable: r.get(5),
            is_primary_key: r.get(6),
            is_updatable: r.get(7),
        })
        .collect();

    //group the rows into vecs for each table
    let mut buckets = HashMap::new();
    for row in rows {
        let key = row.ident();
        let bucket = buckets.entry(key).or_insert_with(|| Vec::default());
        bucket.push(row);
    }

    // build a table for each bucket
    let mut tables = Vec::default();
    for (ident, bucket) in buckets.drain() {
        let ty = bucket[0].kind();
        let columns = build_cols(bucket);
        tables.push(TableDef {
            ident,
            ty,
            columns,
            has_many: Vec::default(),
            belongs_to: Vec::default(),
        });
    }

    // Build a list of all the FKs
    let sql = DB::fk_scan_sql();
    let q = sqlx::query(sql);
    let fks: Vec<_> = q
        .fetch_all(exec)
        .await?
        .iter()
        .map(|r| FkScanRow {
            me: FkScanTableCol::new(r.get(0), r.get(1), r.get(2)),
            other: FkScanTableCol::new(r.get(3), r.get(4), r.get(5)),
        })
        .collect();
    // Build lookup to the FKs
    let mut belongs_to = build_lookup(&fks, |x| &x.me);
    let mut has_many = build_lookup(&fks, |x| &x.other);

    // Add all the FKs to their appropriate tables
    for table in &mut tables {
        let ident = table.ident.clone();
        // build the belongs_to
        if let Some(bt) = belongs_to.remove(&ident) {
            bt.iter().for_each(|&x| {
                let other_table = x.other.ident.clone();
                let fk = x.me.column.as_str();
                let pk = x.other.column.as_str();
                let ref_def = RelationDef::new(other_table, fk, pk);
                table.belongs_to.push(ref_def);
            });
        }
        // has_many
        if let Some(hm) = has_many.remove(&ident) {
            hm.iter().for_each(|&x| {
                let other_table = x.me.ident.clone();
                let fk = x.me.column.as_str();
                let pk = x.other.column.as_str();
                let ref_def = RelationDef::new(other_table, fk, pk);
                table.has_many.push(ref_def);
            });
        }
    }

    Ok(tables)
}

fn build_lookup<'a>(
    fks: &'a [FkScanRow],
    src: impl Fn(&FkScanRow) -> &FkScanTableCol,
) -> HashMap<&'a TableIdent, Vec<&'a FkScanRow>> {
    let mut map = HashMap::new();
    for fk in fks {
        let key = &src(fk).ident;
        let values = map.entry(key).or_insert_with(|| Vec::default());
        values.push(fk);
    }
    map
}

fn build_cols(mut rows: Vec<TableScanRow>) -> Vec<ColumnDef> {
    rows.drain(..)
        .map(|r| ColumnDef {
            name: r.column_name,
            ty: r.column_type,
            null: r.is_nullable > 0,
            primary_key: r.is_primary_key > 0,
            updatable: r.is_updatable > 0,
        })
        .collect()
}