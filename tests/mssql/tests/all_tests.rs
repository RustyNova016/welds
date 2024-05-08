use mssql_test::models::order::Order;
use mssql_test::models::product::{BadProductColumns, BadProductMissingTable, Product};
use mssql_test::models::StringThing;
use mssql_test::models::Thing1;
use welds::connections::mssql::MssqlClient;
use welds::state::{DbState, DbStatus};
use welds::TransactStart;
use welds::{Client, Syntax};

mod migrations;

async fn get_conn() -> MssqlClient {
    let conn = testlib::mssql::conn().await.unwrap();
    let client: MssqlClient = conn.into();
    client
}

#[derive(Default, Debug, Clone)]
pub struct Test {
    pub id: i32,
}

#[tokio::test]
async fn should_be_able_to_connect() {
    let conn = get_conn().await;
    assert!(true);
}

#[tokio::test]
async fn should_select_with_raw_connection() {
    let mut conn = get_conn().await;
    let sql = "SELECT id FROM welds.products where id != @p1 AND id != @p2";
    let rows = conn.fetch_rows(&sql, &[&41, &43]).await;
    let rows = rows.unwrap();
    assert!(!rows.is_empty());
}

#[tokio::test]
async fn should_be_able_to_read_all_products() {
    let conn = get_conn().await;
    let q = Product::all();
    eprintln!("SQL: {}", q.to_sql(Syntax::Mssql));
    let all = q.run(&conn).await.unwrap();
    assert_eq!(all.len(), 6, "Unexpected number of rows returned");
}

#[tokio::test]
async fn should_be_able_to_filter_on_equal() {
    let conn = get_conn().await;
    let q = Product::where_col(|x| x.price_1.equal(1.10));
    eprintln!("SQL: {}", q.to_sql(Syntax::Mssql));
    let just_horse = q.run(&conn).await.unwrap();
    assert_eq!(
        just_horse.len(),
        1,
        "Expected to only find the horse in the test data"
    );
}

#[tokio::test]
async fn should_be_able_to_filter_on_lt() {
    let conn = get_conn().await;
    let q = Product::where_col(|x| x.price_1.lt(3.00));
    eprintln!("SQL: {}", q.to_sql(Syntax::Mssql));
    let data = q.run(&conn).await.unwrap();
    assert_eq!(data.len(), 2, "Expected horse and dog",);
}

#[tokio::test]
async fn should_be_able_to_filter_on_lte() {
    let conn = get_conn().await;
    let q = Product::where_col(|x| x.price_1.lte(2.10));
    eprintln!("SQL: {}", q.to_sql(Syntax::Mssql));
    let data = q.run(&conn).await.unwrap();
    assert_eq!(data.len(), 2, "Expected horse and dog");
}

#[tokio::test]
async fn should_be_able_to_count_in_sql() {
    let conn = get_conn().await;
    let q = Product::where_col(|x| x.price_1.lte(2.10));
    eprintln!("SQL: {}", q.to_sql(Syntax::Mssql));
    let count = q.count(&conn).await.unwrap();
    assert_eq!(count, 2,);
}

#[tokio::test]
async fn should_be_able_to_limit_results_in_sql() {
    let conn = get_conn().await;
    let q = Product::all().limit(2).offset(1);
    eprintln!("SQL: {}", q.to_sql(Syntax::Mssql));
    let count = q.run(&conn).await.unwrap().len();
    assert_eq!(count, 2);
}

#[tokio::test]
async fn should_be_able_to_create_a_new_product() {
    let conn = get_conn().await;
    let trans = conn.begin().await.unwrap();

    let mut p1 = Product::new();
    p1.name = "newyNewFace".to_owned();
    p1.description = Some("YES!".to_owned());
    // Note: creation will set the PK for the model.
    p1.save(&trans).await.unwrap();

    let q = Product::where_col(|x| x.id.equal(p1.id));
    let mut found: Vec<_> = q.run(&trans).await.unwrap();
    let p2 = found.pop().unwrap();
    assert_eq!(p2.name, "newyNewFace");
    assert!(p2.id != 0, "New ID should not be Zero");

    trans.rollback().await.unwrap();
}

#[tokio::test]
async fn should_be_able_to_update_a_product() {
    let conn = get_conn().await;
    let trans = conn.begin().await.unwrap();

    let mut p1 = Product::all()
        .order_by_desc(|x| x.id)
        .limit(1)
        .run(&trans)
        .await
        .unwrap()
        .pop()
        .unwrap();

    p1.description = Some("UPDATED!".to_owned());
    p1.save(&trans).await.unwrap();
    let p2 = Product::find_by_id(&trans, p1.id).await.unwrap().unwrap();
    assert_eq!(p2.description.as_ref().unwrap(), "UPDATED!");
    trans.rollback().await.unwrap();
}

#[tokio::test]
async fn should_be_able_to_filter_on_relations() {
    let conn = get_conn().await;
    let orders = Product::where_col(|x| x.id.equal(1)).map_query(|p| p.order);
    let orders = orders.run(&conn).await.unwrap();
    assert_eq!(3, orders.len());
}

#[tokio::test]
async fn should_be_able_to_filter_on_relations2() {
    let conn = get_conn().await;
    let product_query = Order::all().map_query(|p| p.product);
    // Vec<_> would be simpler, but want to hard code to type for test.
    use welds::state::DbState;
    let products: Vec<DbState<Product>> = product_query.run(&conn).await.unwrap();
    assert_eq!(2, products.len());
}

#[tokio::test]
async fn should_be_able_to_filter_with_relations() {
    let conn = get_conn().await;
    let filter1 = Product::where_col(|x| x.id.equal(1));
    let mut order_query = Order::all();
    order_query = order_query.where_relation(|o| o.product, filter1);
    // Vec<_> would be simpler, but want to hard code to type for test.
    use welds::state::DbState;
    let orders: Vec<DbState<Order>> = order_query.run(&conn).await.unwrap();
    assert_eq!(3, orders.len());
}

#[tokio::test]
async fn should_be_able_to_filter_with_relations2() {
    let conn = get_conn().await;
    let filter1 = Order::where_col(|x| x.id.lte(3));
    let mut product_query = Product::all();
    product_query = product_query.where_relation(|p| p.order, filter1);
    // Vec<_> would be simpler, but want to hard code to type for test.
    use welds::state::DbState;
    let orders: Vec<DbState<Product>> = product_query.run(&conn).await.unwrap();
    assert_eq!(1, orders.len());
}

#[tokio::test]
async fn should_be_able_to_scan_for_all_tables() {
    let conn = get_conn().await;
    let tables = welds::detect::find_tables(&conn).await.unwrap();
    assert!(tables.len() >= 14);
}

#[tokio::test]
async fn a_model_should_be_able_to_verify_its_schema_missing_table() {
    let conn = get_conn().await;
    let issues = welds::check::schema::<BadProductMissingTable>(&conn)
        .await
        .unwrap();
    assert_eq!(issues.len(), 1);
    let issue = &issues[0];
    assert_eq!(issue.kind, welds::check::Kind::MissingTable);
}

#[tokio::test]
async fn a_model_should_be_able_to_verify_its_schema_missing_column() {
    let conn = get_conn().await;
    let issues = welds::check::schema::<BadProductColumns>(&conn)
        .await
        .unwrap();
    // NOTE: a column name changed so it is added on the model and removed in the db giving two warnings
    for issue in &issues {
        eprintln!("{}", issue);
    }
    assert_eq!(issues.len(), 6);
}

#[tokio::test]
async fn should_be_able_to_bulk_delete() {
    let conn = get_conn().await;
    let trans = conn.begin().await.unwrap();
    let p1 = Product::all()
        .limit(1)
        .run(&trans)
        .await
        .unwrap()
        .pop()
        .unwrap();
    let mut order = Order::new();
    order.product_id = p1.id;
    order.save(&trans).await.unwrap();
    let q = Order::all().where_col(|x| x.id.gt(0));
    let count = q.count(&trans).await.unwrap();
    q.delete(&trans).await.unwrap();
    assert!(count > 0);
    trans.rollback().await.unwrap();
}

#[tokio::test]
async fn should_be_able_to_bulk_update() {
    let conn = get_conn().await;
    let q = Product::all().set(|x| x.description, "thing");
    let sql = q.to_sql(Syntax::Mssql);
    eprintln!("SQL: {}", sql);

    //let q = Order::all()
    //    .where_col(|x| x.code.equal(None))
    //    .set(|x| x.code, "test");
    q.run(&conn).await.unwrap();
}

#[tokio::test]
async fn should_be_able_to_bulk_insert() {
    let conn = get_conn().await;
    let trans = conn.begin().await.unwrap();
    let things: Vec<_> = (0..3000)
        .map(|x| Thing1 {
            id: 0,
            value: format!("Bulk_Insert: {}", x),
        })
        .collect();
    welds::query::insert::bulk_insert(&trans, &things)
        .await
        .unwrap();
    trans.rollback().await.unwrap();
}

#[tokio::test]
async fn should_be_able_to_create_a_model_with_a_string_id() {
    let conn = get_conn().await;
    let mut thing = DbState::new_uncreated(StringThing {
        id: "test".to_owned(),
        value: "test".to_owned(),
    });
    thing.save(&conn).await.unwrap();
    assert_eq!(thing.db_status(), DbStatus::NotModified);
    let found = StringThing::find_by_id(&conn, "test".to_owned())
        .await
        .unwrap();
    assert!(found.is_some());
}
