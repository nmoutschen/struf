use struf::Filter;

#[derive(Filter)]
pub struct MyStruct {
    #[filter]
    pub name: String,
}

#[test]
fn test_single() {
    let filter = MyStruct::filter().with_name("my_name");
    assert!(filter.matches(&MyStruct {
        name: "my_name".to_string()
    }));
    assert!(!filter.matches(&MyStruct {
        name: "not_my_name".to_string()
    }));
}

#[test]
fn test_multiple() {
    let filter = MyStruct::filter().with_names(vec!["name_a", "name_b"]);
    assert!(filter.matches(&MyStruct {
        name: "name_a".to_string()
    }));
    assert!(filter.matches(&MyStruct {
        name: "name_b".to_string()
    }));
    assert!(!filter.matches(&MyStruct {
        name: "name_c".to_string()
    }));
}