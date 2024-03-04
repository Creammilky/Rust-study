//Field : Type
//注意，Struct拥有其所有非引用数据的所有权，其随着struct的消亡而消亡，引用类型的字段需要考虑生命周期
struct User {
    username : String ,
    email : String,
    sign_in_count : u64,
    active : bool,
}

//具名元组可以使用struct来定义，这样的元组具有类型:结构名(i.e. Color, Point3d , &et)， 它们的类型是是不同的
struct Color(i32,i32,i32);
struct Point3d(i32, i32, i32);

//空结构体类型也可以被定义，叫Unit-Like Struct，用于在某个类型上面实现接口(trait)
struct Unit{}
fn main() {
    let user_immutable = User{
        username : String::from("Carl Tan"),
        email : String::from("carl_tan_fake @ gmail.com"),
        active: true,
        sign_in_count: 1,
    };
    // all fields are immutable
    println!("User : {}; Email : {}.", user_immutable.username, user_immutable.email);

    let mut user_mutable : User = User{
        username: "Phew Doe".to_string(),
        email: "phew_doe_fake @ gmail.com".to_string(),
        sign_in_count: 0,
        active: false,
    };
    // Once mut is defined, all fields are mutable,
    user_mutable.email = "phew_doe_muted @ gmail.cum".to_string();
    println!("User : {}; Email : {}.", user_mutable.username, user_mutable.email);

    // A new Instance of struct User also can be created in a func as return
    let user_fn_create: User = get_user("John Doe", "john_doe_fake @ gmail.com");
    println!("User : {}; Email : {}.", user_fn_create.username, user_fn_create.email);

    let user_dup = dup_user_from_other("Puss Dick".to_string(), "pussy_dick @ gmail.com".to_string(), user_immutable);
    println!("User : {}; Email : {}; active : {}, count : {}", user_dup.username, user_dup.email, user_dup.active, user_dup.sign_in_count);
}

// A new Instance of struct User also can be created in a func as return
/* Note that this is a way to "initialize" a struct with given value,
 while you can't set default value in the prototype of which */
fn get_user(username: &str, email: &str) -> User {
    User{
        username: username.to_string(),
        email: email.to_string(),
        sign_in_count: 0,
        active: false,
    }
}

// when the parameter in fn have the same name and type! with struct are about to init, we can use this simple way.
fn get_user_by_param(username: String, email: String) -> User {
    User{
        username,
        email,
        sign_in_count: 0,
        active: false,
    }
}

/* ..another_instance_of_same_struct,
 can easily create one instance and keep fields that has not been given same as another */
fn dup_user_from_other(username: String, email: String, template: User) ->User{
    User{
        username,
        email,
        ..template  // Here!
    }
}