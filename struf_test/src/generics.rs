use struf::Filter;

#[derive(Filter)]
pub struct Generics<'a, A, B> {
    #[filter]
    pub name_a: &'a A,
    #[filter]
    pub name_b: B,
}

#[test]
fn test_single() {
    let name_a = "name_a".to_string();
    let filter = Generics::<'_, _, ()>::filter().with_name_a(&name_a);
    // Match on A
    assert!(filter.matches(&Generics {
        name_a: &name_a,
        name_b: ()
    }));
    // No match on A
    assert!(!filter.matches(&Generics {
        name_a: &"not_name_a".to_string(),
        name_b: ()
    }));
}

#[test]
fn test_multiple() {
    let name_1 = "name_1".to_string();
    let name_2 = "name_2".to_string();
    let filter = Generics::filter()
        .with_name_as(vec![&name_1, &name_2])
        .with_name_b(name_1.clone());

    // Match on A and B
    assert!(filter.matches(&Generics {
        name_a: &name_1,
        name_b: name_1.clone()
    }));

    // Match on A but not B
    assert!(!filter.matches(&Generics {
        name_a: &name_1,
        name_b: name_2.clone()
    }));

    // Match on B but no A
    assert!(!filter.matches(&Generics {
        name_a: &"not_name_a".to_string(),
        name_b: name_1.clone()
    }));
}
