//use my_macros::say_hello;
use my_macros::ToSql;

#[derive(ToSql)]
struct User {
    id: i32,
    name: String,
    age: i32,
}
fn main() {
    //say_hello!("Привет из процедурного макроса!");

    let user = User {
        id: 1,
        name: "Alice".into(),
        age: 30,
    };
    println!("{}", user.to_sql("users"));
}
