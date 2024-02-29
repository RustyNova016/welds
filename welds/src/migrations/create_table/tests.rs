use super::*;
use crate::migrations::MigrationWriter;
use crate::Syntax;

#[test]
fn should_create_basic_table() {
    let m = create_table("s1.MyTable")
        .id(|c| c("id", Type::Int))
        .column(|c| c("name", Type::String));

    //mysql
    let sql = MigrationWriter::up_sql(&m, Syntax::Mysql).join("; ");
    let expected = r#"
    CREATE TABLE s1.MyTable ( id INT AUTO_INCREMENT PRIMARY KEY, name VARCHAR(255) NOT NULL )"#;
    assert_eq!(sql, expected.trim());

    //postgres
    let sql2 = MigrationWriter::up_sql(&m, Syntax::Postgres).join("; ");
    let expected = r#"
    CREATE TABLE s1.MyTable ( id SERIAL PRIMARY KEY, name TEXT NOT NULL )"#;
    assert_eq!(sql2, expected.trim());

    //mysql
    let sql = MigrationWriter::up_sql(&m, Syntax::Mssql).join("; ");
    let expected = r#"
    CREATE TABLE s1.MyTable ( id INT IDENTITY(1,1) PRIMARY KEY, name NVARCHAR(MAX) NOT NULL )"#;
    assert_eq!(sql, expected.trim());

    //sqlite
    let sql2 = MigrationWriter::up_sql(&m, Syntax::Sqlite).join("; ");
    let expected = r#"
    CREATE TABLE s1.MyTable ( id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL )"#;
    assert_eq!(sql2, expected.trim());
}

#[test]
fn should_drop_basic_table() {
    let m = create_table("s1.MyTable")
        .id(|c| c("id", Type::Int))
        .column(|c| c("name", Type::String));
    let expected = r#"DROP TABLE s1.MyTable"#;

    //mysql
    let sql = MigrationWriter::down_sql(&m, Syntax::Mysql).join("; ");
    assert_eq!(sql, expected.trim());

    //postgres
    let sql2 = MigrationWriter::down_sql(&m, Syntax::Postgres).join("; ");
    assert_eq!(sql2, expected.trim());
}
