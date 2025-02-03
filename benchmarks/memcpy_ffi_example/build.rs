fn main() {
    cc::Build::new()
        .file("memcpy_example.c")
        .compile("memcpy_example");
}

