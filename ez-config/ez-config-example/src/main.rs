use ez_config_env::MyTrait;

trait MyTrait {
    fn answer() -> i32 {
        42
    }
}

#[derive(MyTrait)]
struct Foo;

fn main() {
    print!("{}", Foo::answer());
}