extern crate gcc;

fn main() {
    gcc::Config::new().file("src/add.c").compile("libadd.a");
}
