
extern crate core;
mod statement;

pub fn test(){
    statement::statement::test_function();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
