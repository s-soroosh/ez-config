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
    // print!("{}", Foo::answer());
    let foo = Foo {
        name: "Soroosh".to_string()
    };

    println!("{:#?}", foo.name);
}