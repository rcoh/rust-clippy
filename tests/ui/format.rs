
#![allow(print_literal)]
#![warn(useless_format)]

struct Foo(pub String);

macro_rules! foo {
  ($($t:tt)*) => (Foo(format!($($t)*)))
}

fn main() {
    format!("foo");

    format!("{}", "foo");
    format!("{:?}", "foo"); // we only want to warn about `{}`
    format!("{:+}", "foo"); // we only want to warn about `{}`
    format!("foo {}", "bar");
    format!("{} bar", "foo");

    let arg: String = "".to_owned();
    format!("{}", arg);
    format!("{:?}", arg); // we only want to warn about `{}`
    format!("{:+}", arg); // we only want to warn about `{}`
    format!("foo {}", arg);
    format!("{} bar", arg);

    // we don’t want to warn for non-string args, see #697
    format!("{}", 42);
    format!("{:?}", 42);
    format!("{:+}", 42);
    format!("foo {}", 42);
    format!("{} bar", 42);

    // we only want to warn about `format!` itself
    println!("foo");
    println!("{}", "foo");
    println!("foo {}", "foo");
    println!("{}", 42);
    println!("foo {}", 42);

    // A format! inside a macro should not trigger a warning
    foo!("should not warn");
}
