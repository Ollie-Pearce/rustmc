fn main() {
    cc::Build::new()
        .file("hello.c")
        .compile("hello");
}
