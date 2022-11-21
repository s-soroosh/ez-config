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
}

fn main() {
    let foo = Foo::from_env();
    println!("{}", foo.name);
}