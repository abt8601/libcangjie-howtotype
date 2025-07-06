use std::error::Error;

use libcangjie_howtotype::{CangjieCode, CangjieVersion, LibCangjieHowToType};

#[test]
fn test_v3() -> Result<(), Box<dyn Error>> {
    let cangjie = LibCangjieHowToType::new()?;

    let how_to_type = cangjie.how_to_type("屬", CangjieVersion::V3)?;
    assert_eq!(*how_to_type, [CangjieCode::from_radicals("尸卜卜戈")]);

    Ok(())
}

#[test]
fn test_v5() -> Result<(), Box<dyn Error>> {
    let cangjie = LibCangjieHowToType::new()?;

    let how_to_type = cangjie.how_to_type("屬", CangjieVersion::V5)?;
    assert_eq!(*how_to_type, [CangjieCode::from_radicals("尸水田戈")]);

    Ok(())
}
