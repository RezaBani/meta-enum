use meta_enum::{MetaEnum, ParseMetaEnumError};

#[derive(Debug, MetaEnum, Clone, Copy, PartialEq)]
enum SingleEnum {
    SingleVariant,
}

#[test]
fn single_enum() {
    let single_enum = SingleEnum::SingleVariant;
    assert_eq!(SingleEnum::count(), 1);
    assert_eq!(SingleEnum::keys().len(), 1);
    assert_eq!(SingleEnum::values().len(), 1);
    assert_eq!(SingleEnum::values(), vec![0]);
    assert_eq!("SingleVariant".parse(), Ok(single_enum));
    assert_eq!(Into::<SingleEnum>::into(0_u8), single_enum);
}

#[derive(Debug, MetaEnum, Clone, Copy, PartialEq)]
enum SingleEnumWithValue {
    SingleVariant = 5,
}

#[test]
fn single_enum_with_value() {
    let single_enum = SingleEnumWithValue::SingleVariant;
    assert_eq!(SingleEnumWithValue::count(), 1);
    assert_eq!(SingleEnumWithValue::keys().len(), 1);
    assert_eq!(SingleEnumWithValue::values().len(), 1);
    assert_eq!(SingleEnumWithValue::values(), vec![5]);
    assert_eq!("SingleVariant".parse(), Ok(single_enum));
    assert_eq!(Into::<SingleEnumWithValue>::into(5_u8), single_enum);
}

#[derive(Debug, MetaEnum, Clone, Copy, PartialEq)]
enum MultiEnum {
    VariantOne,
    SecondVariant,
    Variant3,
    SomeThing,
}

#[test]
fn multi_variant() {
    let var_1 = MultiEnum::VariantOne;
    assert_eq!(MultiEnum::count(), 4);
    assert_eq!(MultiEnum::keys().len(), 4);
    assert_eq!(MultiEnum::values().len(), 4);
    assert_eq!(MultiEnum::values(), vec![0, 1, 2, 3]);
    assert_eq!("VariantOne".parse(), Ok(var_1));
    assert_eq!(Into::<MultiEnum>::into(2_u8), MultiEnum::Variant3);
}

#[derive(Debug, MetaEnum, Clone, Copy, PartialEq)]
enum MultiEnumWithValue {
    VariantOne = 0,
    SecondVariant = 1,
    Variant3 = 2,
    SomeThing = 4,
    OtherAsWell = 8,
}

#[test]
fn multi_variant_with_value() {
    let var_1 = MultiEnumWithValue::OtherAsWell;
    assert_eq!(MultiEnumWithValue::count(), 5);
    assert_eq!(MultiEnumWithValue::keys().len(), 5);
    assert_eq!(MultiEnumWithValue::values().len(), 5);
    assert_eq!(MultiEnumWithValue::values(), vec![0, 1, 2, 4, 8]);
    assert_eq!("otherasWell".parse(), Ok(var_1));
    assert_eq!(
        "Not In Enum".parse::<MultiEnumWithValue>(),
        Err(ParseMetaEnumError::<MultiEnumWithValue>::new())
    );
    assert_eq!(
        Into::<MultiEnumWithValue>::into(4_u8),
        MultiEnumWithValue::SomeThing
    );
}
