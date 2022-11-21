use ez_config_env::EnvConfig;

trait MyTrait {
    fn answer() -> i32 {
        42
    }
}

#[derive(EnvConfig)]
#[derive(Debug)]
struct Foo {
    name: String,
    last_name: String,
    age: u16,
    field_boolean: bool,
}

fn main() {
    let foo = Foo::from_env();
    println!("{:#?}", foo);
}