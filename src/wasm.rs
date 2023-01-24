extern crate console_error_panic_hook;

use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

use js_sys::{Object, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, HtmlCanvasElement};

use crate::{
  keyboard::KeyMappingStrategy,
  platform::{AsyncPlatform, CanvasPlatform, Platform},
  systems::{
    c64::{C64SystemBuilder, C64SystemConfig, C64SystemRoms},
    pet::{PetSystemBuilder, PetSystemConfig, PetSystemRoms},
    vic::{Vic20SystemBuilder, Vic20SystemConfig, Vic20SystemRoms},
    System, SystemBuilder,
  },
};

#[wasm_bindgen]
pub struct NoentiendoBuilder {
  canvas: Option<HtmlCanvasElement>,
  roms: Option<Object>,
  system: Option<String>,
}

#[wasm_bindgen]
impl NoentiendoBuilder {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    Self {
      canvas: None,
      roms: None,
      system: None,
    }
  }

  pub fn with_canvas(mut self, canvas: HtmlCanvasElement) -> Self {
    self.canvas = Some(canvas);
    self
  }

  pub fn with_roms(mut self, roms: Object) -> Self {
    self.roms = Some(roms);
    self
  }

  pub fn with_system(mut self, system: String) -> Self {
    self.system = Some(system);
    self
  }

  pub fn build(&self) -> Noentiendo {
    let canvas = self.canvas.as_ref().expect("Canvas not set");
    let platform = CanvasPlatform::new(canvas.clone());

    let roms = self.roms.as_ref().expect("Roms not set");

    let pet_roms = Reflect::get(roms, &JsValue::from_str("pet")).unwrap();
    let pet_roms = PetSystemRoms::from_jsvalue(&pet_roms);

    let vic_roms = Reflect::get(roms, &JsValue::from_str("vic")).unwrap();
    let vic_roms = Vic20SystemRoms::from_jsvalue(&vic_roms);

    let c64_roms = Reflect::get(roms, &JsValue::from_str("c64")).unwrap();
    let c64_roms = C64SystemRoms::from_jsvalue(&c64_roms);

    let system = self.system.as_ref().expect("System not set");

    let system = match system.as_str() {
      "pet" => PetSystemBuilder::build(
        pet_roms,
        PetSystemConfig {
          mapping: KeyMappingStrategy::Symbolic,
        },
        platform.provider(),
      ),
      "vic" => Vic20SystemBuilder::build(
        vic_roms,
        Vic20SystemConfig {
          mapping: KeyMappingStrategy::Symbolic,
        },
        platform.provider(),
      ),
      "c64" => C64SystemBuilder::build(
        c64_roms,
        C64SystemConfig {
          mapping: KeyMappingStrategy::Symbolic,
        },
        platform.provider(),
      ),
      _ => panic!("Unknown system"),
    };

    Noentiendo::new(platform, system)
  }
}

#[wasm_bindgen]
pub struct Noentiendo {
  interval_id: i32,
}

#[wasm_bindgen]
impl Noentiendo {
  fn new(platform: CanvasPlatform, system: Box<dyn System>) -> Self {
    console_error_panic_hook::set_once();

    let platform = Rc::new(RefCell::new(platform));
    let platform_ready = Rc::new(Cell::new(false));

    {
      let platform = platform.clone();
      let platform_ready = platform_ready.clone();
      spawn_local(async move {
        platform.borrow_mut().setup().await;
        platform_ready.set(true);
      });
    }

    let system = Rc::new(RefCell::new(system));

    let handler: Box<dyn FnMut() -> ()> = Box::new(move || {
      if platform_ready.get() {
        let platform = platform.clone();
        let system = system.clone();
        spawn_local(async move {
          let platform = platform.try_borrow_mut();

          if let Ok(mut platform) = platform {
            platform.tick(&mut Box::new(system.borrow_mut())).await;
          } else {
            web_sys::console::log_1(&"can't borrow!".into());
          }
        });
      }
    });

    let handle_tick = Closure::new(handler);

    let interval_id = window()
      .unwrap()
      .set_interval_with_callback_and_timeout_and_arguments_0(
        handle_tick.as_ref().unchecked_ref(),
        20,
      )
      .unwrap();

    handle_tick.forget();

    Self { interval_id }
  }

  pub fn close(&mut self) {
    window()
      .unwrap()
      .clear_interval_with_handle(self.interval_id);
  }

  pub fn reset(&mut self) {
    // todo!();
  }

  pub fn dispatch_key(&mut self, key: String, down: bool) {
    todo!();
  }
}

// #[cfg(target_arch = "wasm32")]
// #[wasm_bindgen]
// pub fn main(roms: &JsValue, system: &JsValue) {
//   console_error_panic_hook::set_once();

//   use js_sys::Reflect;
//   use keyboard::KeyMappingStrategy;
//   use platform::{AsyncPlatform, CanvasPlatform, Platform};
//   use systems::{
//     pet::PetSystemBuilder, pet::PetSystemConfig, pet::PetSystemRoms, vic::Vic20SystemBuilder,
//     vic::Vic20SystemConfig, vic::Vic20SystemRoms, SystemBuilder,
//   };
//   use wasm_bindgen_futures::spawn_local;

//   let mut platform = CanvasPlatform::new();

//   let pet_object = Reflect::get(&roms, &JsValue::from_str("pet")).unwrap();
//   let vic_object = Reflect::get(&roms, &JsValue::from_str("vic")).unwrap();

//   let pet_roms = PetSystemRoms::from_jsvalue(&pet_object);
//   let vic_roms = Vic20SystemRoms::from_jsvalue(&vic_object);

//   let system = match system.as_string().unwrap().as_str() {
//     "pet" => PetSystemBuilder::build(
//       pet_roms,
//       PetSystemConfig {
//         mapping: KeyMappingStrategy::Symbolic,
//       },
//       platform.provider(),
//     ),
//     "vic" => Vic20SystemBuilder::build(
//       vic_roms,
//       Vic20SystemConfig {
//         mapping: KeyMappingStrategy::Symbolic,
//       },
//       platform.provider(),
//     ),
//     _ => panic!("Unknown system"),
//   };

//   spawn_local(async move {
//     platform.run_async(system).await;
//   });
// }
