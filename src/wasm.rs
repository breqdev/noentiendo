extern crate console_error_panic_hook;

use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

use js_sys::{Object, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, HtmlCanvasElement};

use crate::keyboard::KeyState;
use crate::keyboard::VirtualKey;
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
    console_error_panic_hook::set_once();

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
    let virtual_key_state = Arc::new(Mutex::new(KeyState::new()));
    let platform = CanvasPlatform::new(canvas.clone(), virtual_key_state.clone());

    let roms = self.roms.as_ref().expect("Roms not set");

    let pet_roms = Reflect::get(roms, &JsValue::from_str("pet")).unwrap();
    let pet_roms = PetSystemRoms::from_jsvalue(&pet_roms);

    let vic_roms = Reflect::get(roms, &JsValue::from_str("vic")).unwrap();
    let vic_roms = Vic20SystemRoms::from_jsvalue(&vic_roms);

    let c64_roms = Reflect::get(roms, &JsValue::from_str("c64")).unwrap();
    let c64_roms = C64SystemRoms::from_jsvalue(&c64_roms);

    let system = self.system.as_ref().expect("System not set");

    let mut system = match system.as_str() {
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

    system.reset();

    Noentiendo::new(platform, system, virtual_key_state)
  }
}

#[wasm_bindgen]
pub struct Noentiendo {
  interval_id: i32,
  system: Rc<RefCell<Box<dyn System>>>,
  virtual_keys: Arc<Mutex<KeyState<VirtualKey>>>,
}

#[wasm_bindgen]
impl Noentiendo {
  fn new(
    platform: CanvasPlatform,
    system: Box<dyn System>,
    virtual_key_state: Arc<Mutex<KeyState<VirtualKey>>>,
  ) -> Self {
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

    let interval_id = {
      let system = system.clone();
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

      interval_id
    };

    Self {
      interval_id,
      system,
      virtual_keys: virtual_key_state,
    }
  }

  pub fn close(&mut self) {
    window()
      .unwrap()
      .clear_interval_with_handle(self.interval_id);
  }

  pub fn reset(&mut self) {
    self.system.borrow_mut().reset();
  }

  pub fn dispatch_key(&mut self, key: JsValue, down: bool) {
    if down {
      self
        .virtual_keys
        .lock()
        .unwrap()
        .press(serde_wasm_bindgen::from_value(key).unwrap());
    } else {
      self
        .virtual_keys
        .lock()
        .unwrap()
        .release(serde_wasm_bindgen::from_value(key).unwrap());
    }
  }
}
