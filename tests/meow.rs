use std::error::Error;

use libcangjie_howtotype::{CangjieCode, CangjieVersion, LibCangjieHowToType};

#[test]
fn meow() -> Result<(), Box<dyn Error>> {
    let cangjie = LibCangjieHowToType::new()?;

    let how_to_type = cangjie.how_to_type("喵", CangjieVersion::V3)?;
    assert_eq!(*how_to_type, [CangjieCode::from_radicals("口廿田")]);

    Ok(())
}
