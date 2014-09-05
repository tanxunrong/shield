
pub struct Query {
    orig:String,
    table:String
}

pub fn new_sql(q:&str) -> Query {
    Query{orig:String::from_str(q),table:String::new()}
}

#[test]
fn test_init_sql() {
    let q = "select * from t_bag where uid = 20111;";
    let mut s =  new_sql(q);
}
