include!(concat!(env!("OUT_DIR"), concat!("/", "test", ".rs")));

#[cfg(test)]
mod tests {
    use crate::JoinRequest;
    #[test]
    fn it_works() {
        let req = JoinRequest{id:1,host:"test".into()};
       // crate::generate_issue();
        println!("{:?}",req);
        assert_eq!(2 + 2, 4);
    }
}

