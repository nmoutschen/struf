use struf::Filter;

#[derive(Filter)]
pub struct Basic {
    #[filter]
    pub name: String,
}

#[test]
fn test_single() {
    let filter = Basic::filter().with_name("my_name");
    assert!(filter.matches(&Basic {
        name: "my_name".to_string()
    }));
    assert!(!filter.matches(&Basic {
        name: "not_my_name".to_string()
    }));
}

#[test]
fn test_multiple() {
    let filter = Basic::filter().with_names(vec!["name_a", "name_b"]);
    assert!(filter.matches(&Basic {
        name: "name_a".to_string()
    }));
    assert!(filter.matches(&Basic {
        name: "name_b".to_string()
    }));
    assert!(!filter.matches(&Basic {
        name: "name_c".to_string()
    }));
}
