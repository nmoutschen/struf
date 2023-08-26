use struf::Filter;

#[derive(Filter)]
pub struct MyStruct {
    #[filter]
    pub name: String,
}

#[test]
fn test_my_struct() {
    let filter = MyStruct::filter().with_name("my_name".to_string());
    assert!(filter.matches(&MyStruct {
        name: "my_name".to_string()
    }));
    assert!(!filter.matches(&MyStruct {
        name: "not_my_name".to_string()
    }));
}
