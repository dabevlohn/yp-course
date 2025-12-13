use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

//use my_macros::say_hello;
use my_macros::{FromSql, ToSql};

#[derive(Debug)]
enum Status {
    Online,
    Offline,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Status::Online => write!(f, "Online"),
            Status::Offline => write!(f, "Offline"),
        }
    }
}

impl FromStr for Status {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Online" => Ok(Status::Online),
            "Offline" => Ok(Status::Offline),
            _ => Err(format!("Unknown status: {}", s)),
        }
    }
}

#[derive(Debug, ToSql, FromSql)]
struct User {
    id: i32,
    name: String,
    age: i32,
    status: Status,
}
fn main() {
    //say_hello!("Привет из процедурного макроса!");

    let user = User {
        id: 1,
        name: "Alice".into(),
        age: 30,
        status: Status::Online,
    };
    println!("{}", user.to_sql("users"));

    let sql = "INSERT INTO users (id, name,age, status) VALUES('1','Bob','35', 'Offline');";
    let user2 = User::from_sql(sql);
    println!("{:?}", user2);
}
