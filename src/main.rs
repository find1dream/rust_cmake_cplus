extern crate cmake_project;

fn main() {
    unsafe {
        cmake_project::foo();
        let a: i32 = 5;
        let b: i32 = 5;
        let c = cmake_project::add(a, b);
        assert_eq!(10, c);
        println!(" a + b = {}", c);

    }
}
