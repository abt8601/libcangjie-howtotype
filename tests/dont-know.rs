use std::error::Error;

use libcangjie_howtotype::{CangjieVersion, LibCangjieHowToType};

#[test]
fn test_dont_know() -> Result<(), Box<dyn Error>> {
    let cangjie = LibCangjieHowToType::new()?;

    let how_to_type = cangjie.how_to_type("😀", CangjieVersion::V3)?;
    assert!(how_to_type.is_empty());

    Ok(())
}
