trait A {
    fn a(&self) -> String;
}
fn test() {
    #[nostrumock_macros::mock(let a = "a".into())]
    impl A for String {
        fn a(&self) -> String {
            unimplemented!()
        }
    }
}
