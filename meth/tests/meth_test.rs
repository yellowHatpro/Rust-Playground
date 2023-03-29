#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = meth::pow(2,2);
        let div = match meth::divide(5,5) {
            Ok(ans) => println!("Result: {}",ans),
            Err(e) => println!("Error: {}",e),

        };
        assert_eq!(result, 4);
    }
}
