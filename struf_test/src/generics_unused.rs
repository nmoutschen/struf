use struf::Filter;

#[derive(Filter)]
pub struct Generics<'a, A, B> {
    pub name_a: &'a A,
    #[filter]
    pub name_b: B,
}

#[test]
fn test_single() {
    let name_b = "name_b".to_string();
    let filter = Generics::filter().with_name_b(name_b.clone());
    // Match on B
    assert!(filter.matches(&Generics {
        name_a: &"",
        name_b: name_b.clone(),
    }));
    // No match on B
    assert!(!filter.matches(&Generics {
        name_a: &"",
        name_b: "not b".to_string(),
    }));
}

#[test]
fn test_multiple() {
    let name_1 = "name_1".to_string();
    let name_2 = "name_2".to_string();
    let filter = Generics::filter().with_name_bs(vec![&name_1, &name_2]);

    // Match
    assert!(filter.matches(&Generics {
        name_a: &3,
        name_b: name_1.clone()
    }));

    // Match
    assert!(filter.matches(&Generics {
        name_a: &3,
        name_b: name_2.clone()
    }));

    // Does not match
    assert!(!filter.matches(&Generics {
        name_a: &3,
        name_b: "not b".to_string()
    }));
}
