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

#[cfg(test)]
mod test{
    #[test]
    fn test_c_plus_func(){
        unsafe{
            assert_eq!(4, cmake_project::add(1, 3));
        }
    }
}
