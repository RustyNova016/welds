// Warning this file was generated by `gen.rb`

#[cfg(any(feature = "mysql", feature = "sqlite", feature = "postgres"))]
use sqlx::encode::Encode;
#[cfg(any(feature = "mysql", feature = "sqlite", feature = "postgres"))]
use sqlx::types::Type;

#[cfg(feature = "sqlite")]
use super::sqlite::SqliteParam;

#[cfg(feature = "mssql")]
use super::mssql::MssqlParam;

#[cfg(feature = "postgres")]
use super::postgres::PostgresParam;

#[cfg(feature = "mysql")]
use super::mysql::MysqlParam;

#[cfg(all(
    feature = "sqlite",
    not(feature = "postgres"),
    not(feature = "mysql"),
    not(feature = "mssql")
))]
pub trait Param: SqliteParam {}

#[cfg(all(
    feature = "sqlite",
    not(feature = "postgres"),
    not(feature = "mysql"),
    not(feature = "mssql")
))]
impl<T> Param for T
where
    for<'a> T: 'a + Send + Encode<'a, sqlx::Sqlite> + Type<sqlx::Sqlite>,
    for<'a> &'a T: Send,
{
}

#[cfg(all(
    feature = "postgres",
    not(feature = "sqlite"),
    not(feature = "mysql"),
    not(feature = "mssql")
))]
pub trait Param: PostgresParam {}

#[cfg(all(
    feature = "postgres",
    not(feature = "sqlite"),
    not(feature = "mysql"),
    not(feature = "mssql")
))]
impl<T> Param for T
where
    for<'a> T: 'a + Send + Encode<'a, sqlx::Postgres> + Type<sqlx::Postgres>,
    for<'a> &'a T: Send,
{
}

#[cfg(all(
    feature = "mysql",
    not(feature = "sqlite"),
    not(feature = "postgres"),
    not(feature = "mssql")
))]
pub trait Param: MysqlParam {}

#[cfg(all(
    feature = "mysql",
    not(feature = "sqlite"),
    not(feature = "postgres"),
    not(feature = "mssql")
))]
impl<T> Param for T
where
    for<'a> T: 'a + Send + Encode<'a, sqlx::MySql> + Type<sqlx::MySql>,
    for<'a> &'a T: Send,
{
}

#[cfg(all(
    feature = "mssql",
    not(feature = "sqlite"),
    not(feature = "postgres"),
    not(feature = "mysql")
))]
pub trait Param: MssqlParam {}

#[cfg(all(
    feature = "mssql",
    not(feature = "sqlite"),
    not(feature = "postgres"),
    not(feature = "mysql")
))]
impl<T> Param for T
where
    T: MssqlParam,
    for<'a> &'a T: Send,
{
}

#[cfg(all(
    feature = "sqlite",
    feature = "postgres",
    not(feature = "mysql"),
    not(feature = "mssql")
))]
pub trait Param: SqliteParam + PostgresParam {}

#[cfg(all(
    feature = "sqlite",
    feature = "postgres",
    not(feature = "mysql"),
    not(feature = "mssql")
))]
impl<T> Param for T
where
    for<'a> T: 'a
        + Send
        + Encode<'a, sqlx::Sqlite>
        + Type<sqlx::Sqlite>
        + Encode<'a, sqlx::Postgres>
        + Type<sqlx::Postgres>,
    for<'a> &'a T: Send,
{
}

#[cfg(all(
    feature = "sqlite",
    feature = "mysql",
    not(feature = "postgres"),
    not(feature = "mssql")
))]
pub trait Param: SqliteParam + MysqlParam {}

#[cfg(all(
    feature = "sqlite",
    feature = "mysql",
    not(feature = "postgres"),
    not(feature = "mssql")
))]
impl<T> Param for T
where
    for<'a> T: 'a
        + Send
        + Encode<'a, sqlx::Sqlite>
        + Type<sqlx::Sqlite>
        + Encode<'a, sqlx::MySql>
        + Type<sqlx::MySql>,
    for<'a> &'a T: Send,
{
}

#[cfg(all(
    feature = "sqlite",
    feature = "mssql",
    not(feature = "postgres"),
    not(feature = "mysql")
))]
pub trait Param: SqliteParam + MssqlParam {}

#[cfg(all(
    feature = "sqlite",
    feature = "mssql",
    not(feature = "postgres"),
    not(feature = "mysql")
))]
impl<T> Param for T
where
    for<'a> T: 'a + Send + Encode<'a, sqlx::Sqlite> + Type<sqlx::Sqlite> + MssqlParam,
    for<'a> &'a T: Send,
{
}

#[cfg(all(
    feature = "postgres",
    feature = "mysql",
    not(feature = "sqlite"),
    not(feature = "mssql")
))]
pub trait Param: PostgresParam + MysqlParam {}

#[cfg(all(
    feature = "postgres",
    feature = "mysql",
    not(feature = "sqlite"),
    not(feature = "mssql")
))]
impl<T> Param for T
where
    for<'a> T: 'a
        + Send
        + Encode<'a, sqlx::Postgres>
        + Type<sqlx::Postgres>
        + Encode<'a, sqlx::MySql>
        + Type<sqlx::MySql>,
    for<'a> &'a T: Send,
{
}

#[cfg(all(
    feature = "postgres",
    feature = "mssql",
    not(feature = "sqlite"),
    not(feature = "mysql")
))]
pub trait Param: PostgresParam + MssqlParam {}

#[cfg(all(
    feature = "postgres",
    feature = "mssql",
    not(feature = "sqlite"),
    not(feature = "mysql")
))]
impl<T> Param for T
where
    for<'a> T: 'a + Send + Encode<'a, sqlx::Postgres> + Type<sqlx::Postgres> + MssqlParam,
    for<'a> &'a T: Send,
{
}

#[cfg(all(
    feature = "mysql",
    feature = "mssql",
    not(feature = "sqlite"),
    not(feature = "postgres")
))]
pub trait Param: MysqlParam + MssqlParam {}

#[cfg(all(
    feature = "mysql",
    feature = "mssql",
    not(feature = "sqlite"),
    not(feature = "postgres")
))]
impl<T> Param for T
where
    for<'a> T: 'a + Send + Encode<'a, sqlx::MySql> + Type<sqlx::MySql> + MssqlParam,
    for<'a> &'a T: Send,
{
}

#[cfg(all(
    feature = "sqlite",
    feature = "postgres",
    feature = "mysql",
    not(feature = "mssql")
))]
pub trait Param: SqliteParam + PostgresParam + MysqlParam {}

#[cfg(all(
    feature = "sqlite",
    feature = "postgres",
    feature = "mysql",
    not(feature = "mssql")
))]
impl<T> Param for T
where
    for<'a> T: 'a
        + Send
        + Encode<'a, sqlx::Sqlite>
        + Type<sqlx::Sqlite>
        + Encode<'a, sqlx::Postgres>
        + Type<sqlx::Postgres>
        + Encode<'a, sqlx::MySql>
        + Type<sqlx::MySql>,
    for<'a> &'a T: Send,
{
}

#[cfg(all(
    feature = "sqlite",
    feature = "postgres",
    feature = "mssql",
    not(feature = "mysql")
))]
pub trait Param: SqliteParam + PostgresParam + MssqlParam {}

#[cfg(all(
    feature = "sqlite",
    feature = "postgres",
    feature = "mssql",
    not(feature = "mysql")
))]
impl<T> Param for T
where
    for<'a> T: 'a
        + Send
        + Encode<'a, sqlx::Sqlite>
        + Type<sqlx::Sqlite>
        + Encode<'a, sqlx::Postgres>
        + Type<sqlx::Postgres>
        + MssqlParam,
    for<'a> &'a T: Send,
{
}

#[cfg(all(
    feature = "sqlite",
    feature = "mysql",
    feature = "mssql",
    not(feature = "postgres")
))]
pub trait Param: SqliteParam + MysqlParam + MssqlParam {}

#[cfg(all(
    feature = "sqlite",
    feature = "mysql",
    feature = "mssql",
    not(feature = "postgres")
))]
impl<T> Param for T
where
    for<'a> T: 'a
        + Send
        + Encode<'a, sqlx::Sqlite>
        + Type<sqlx::Sqlite>
        + Encode<'a, sqlx::MySql>
        + Type<sqlx::MySql>
        + MssqlParam,
    for<'a> &'a T: Send,
{
}

#[cfg(all(
    feature = "postgres",
    feature = "mysql",
    feature = "mssql",
    not(feature = "sqlite")
))]
pub trait Param: PostgresParam + MysqlParam + MssqlParam {}

#[cfg(all(
    feature = "postgres",
    feature = "mysql",
    feature = "mssql",
    not(feature = "sqlite")
))]
impl<T> Param for T
where
    for<'a> T: 'a
        + Send
        + Encode<'a, sqlx::Postgres>
        + Type<sqlx::Postgres>
        + Encode<'a, sqlx::MySql>
        + Type<sqlx::MySql>
        + MssqlParam,
    for<'a> &'a T: Send,
{
}

#[cfg(all(
    feature = "sqlite",
    feature = "postgres",
    feature = "mysql",
    feature = "mssql"
))]
pub trait Param: SqliteParam + PostgresParam + MysqlParam + MssqlParam {}

#[cfg(all(
    feature = "sqlite",
    feature = "postgres",
    feature = "mysql",
    feature = "mssql"
))]
impl<T> Param for T
where
    for<'a> T: 'a
        + Send
        + Encode<'a, sqlx::Sqlite>
        + Type<sqlx::Sqlite>
        + Encode<'a, sqlx::Postgres>
        + Type<sqlx::Postgres>
        + Encode<'a, sqlx::MySql>
        + Type<sqlx::MySql>
        + MssqlParam,
    for<'a> &'a T: Send,
{
}
