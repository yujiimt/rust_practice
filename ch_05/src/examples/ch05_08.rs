struct UserName(String);
struct Id(u64);
struct Timestamp(u64);
type User = (Id, UserName, Timestamp);

fn new_user(name: UserName, id: Id, created: Timestamp) -> User{
    (id, name, created)
}

fn main(){
    let id = Id(400);
    let now = Timestamp(4567890123);

    // now と id の順番を間違えるとコンパイルエラーになる

    let user = new_user(UserName(String::from("kkkk")), id, now);
}