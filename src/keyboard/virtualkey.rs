use serde::{Deserialize, Serialize};

use crate::systems::pet::PetKeys;

use super::commodore::C64Keys;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VirtualKey {
  Commodore(C64Keys),
  CommodorePet(PetKeys),
  // ...
}
