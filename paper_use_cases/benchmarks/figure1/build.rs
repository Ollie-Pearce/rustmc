fn main() {
    cc::Build::new()
        .file("racy.c")
        .compile("racy");
}
