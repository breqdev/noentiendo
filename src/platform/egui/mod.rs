use crate::keyboard::{winit::WinitAdapter, KeyAdapter, KeyPosition, KeyState, VirtualKey};
use crate::platform::{JoystickState, Platform, PlatformProvider, SyncPlatform, WindowConfig};
use crate::systems::System;
use crate::time::VariableTimeStep;
use egui::{Context, TexturesDelta};
use egui_wgpu::renderer::ScreenDescriptor;
use egui_wgpu::Renderer;
use gilrs::{Button, EventType, Gilrs};
use instant::Duration;
use pixels::{Pixels, SurfaceTexture};
use rand;
use std::io::Write;
use std::sync::{Arc, Mutex};
use wgpu::TextureViewDescriptor;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use super::TapeState;

/// A platform implementation for desktop platforms using Winit and Pixels.
/// This platform runs synchronously.
pub struct EguiPlatform {
  config: Arc<Mutex<Option<WindowConfig>>>,
  provider: Arc<EguiPlatformProvider>,
  key_state: Arc<Mutex<KeyState<VirtualKeyCode>>>,
  joystick_state: Arc<Mutex<JoystickState>>,
  tape_state: Arc<Mutex<TapeState>>,
}

impl EguiPlatform {
  pub fn new() -> Self {
    let config = Arc::new(Mutex::new(None));
    let key_state = Arc::new(Mutex::new(KeyState::new()));
    let joystick_state = Arc::new(Mutex::new(JoystickState::empty()));
    let tape_state = Arc::new(Mutex::new(TapeState::empty()));

    Self {
      provider: Arc::new(EguiPlatformProvider::new(
        config.clone(),
        key_state.clone(),
        joystick_state.clone(),
        tape_state.clone(),
      )),
      config,
      key_state,
      joystick_state,
      tape_state,
    }
  }

  fn get_config(&self) -> WindowConfig {
    let config = self.config.lock().unwrap();
    config.expect("WindowConfig not set")
  }
}

impl Platform for EguiPlatform {
  fn provider(&self) -> Arc<dyn PlatformProvider> {
    self.provider.clone()
  }
}

impl SyncPlatform for EguiPlatform {
  fn run(&mut self, mut system: Box<dyn System>) {
    let event_loop = EventLoop::new();

    let mut current_config = self.get_config();

    let window = WindowBuilder::new()
      .with_title("noentiendo")
      .with_inner_size(LogicalSize::new(
        current_config.width as f64 * current_config.scale,
        current_config.height as f64 * current_config.scale + 50.0,
      ))
      .build(&event_loop)
      .unwrap();

    let inner_size = window.inner_size();
    let scale_factor = window.scale_factor() as f32;

    let surface_texture = SurfaceTexture::new(inner_size.width, inner_size.height, &window);

    let mut pixels =
      Pixels::new(current_config.width, current_config.height, surface_texture).unwrap();

    let max_texture_size = pixels.device().limits().max_texture_dimension_2d as usize;

    let egui_ctx = Context::default();
    egui_ctx.set_visuals(egui::Visuals::light());

    let mut egui_state = egui_winit::State::new(&event_loop);

    egui_state.set_max_texture_side(max_texture_size);
    egui_state.set_pixels_per_point(scale_factor);
    let mut screen_descriptor = ScreenDescriptor {
      size_in_pixels: [inner_size.width, inner_size.height],
      pixels_per_point: scale_factor,
    };
    let mut renderer = Renderer::new(pixels.device(), pixels.render_texture_format(), None, 1);
    let mut textures = TexturesDelta::default();

    let texture = pixels.texture();
    let texture_view = texture.create_view(&TextureViewDescriptor::default());
    let egui_texture = Renderer::register_native_texture(
      &mut renderer,
      pixels.device(),
      &texture_view,
      wgpu::FilterMode::Nearest,
    );

    let mut input = WinitInputHelper::new();
    let key_state = self.key_state.clone();
    let config = self.config.clone();

    system.reset();

    let mut timer = VariableTimeStep::new(Duration::from_secs_f64(1.0 / 60.0));

    let mut gilrs = Gilrs::new().unwrap();
    let joystick_state = self.joystick_state.clone();
    let tape_state = self.tape_state.clone();

    event_loop.run(move |event, _, control_flow| {
      *control_flow = ControlFlow::Poll;

      if input.update(&event) {
        if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() || input.destroyed()
        {
          *control_flow = ControlFlow::Exit;
        }

        if let Some(scale_factor) = input.scale_factor() {
          screen_descriptor.pixels_per_point = scale_factor as f32;
        }

        if let Some(size) = input.window_resized() {
          pixels.resize_surface(size.width, size.height).unwrap();
          if size.width > 0 && size.height > 0 {
            screen_descriptor.size_in_pixels = [size.width, size.height];
          }
        }
      }

      match event {
        Event::MainEventsCleared => {
          timer.do_update(&mut || system.tick());

          {
            let mut joystick_state = joystick_state.lock().unwrap();
            loop {
              let next_event = gilrs.next_event();

              match next_event {
                Some(event) => match event.event {
                  EventType::ButtonPressed(button, _) => match button {
                    Button::DPadLeft => joystick_state.left = true,
                    Button::DPadRight => joystick_state.right = true,
                    Button::DPadUp => joystick_state.up = true,
                    Button::DPadDown => joystick_state.down = true,
                    Button::South => joystick_state.fire = true,
                    _ => {}
                  },
                  EventType::ButtonReleased(button, _) => match button {
                    Button::DPadLeft => joystick_state.left = false,
                    Button::DPadRight => joystick_state.right = false,
                    Button::DPadUp => joystick_state.up = false,
                    Button::DPadDown => joystick_state.down = false,
                    Button::South => joystick_state.fire = false,
                    _ => {}
                  },
                  _ => {}
                },
                None => break,
              }
            }
          }

          {
            let new_config = config.lock().unwrap().unwrap();

            if new_config != current_config {
              current_config = new_config;

              let surface_texture =
                SurfaceTexture::new(inner_size.width, inner_size.height, &window);

              pixels = Pixels::new(new_config.width, new_config.height, surface_texture).unwrap();
            }
          }

          window.request_redraw();
        }

        Event::RedrawRequested(_) => {
          system.render(pixels.get_frame_mut(), config.lock().unwrap().unwrap());

          let raw_input = egui_state.take_egui_input(&window);
          let output = egui_ctx.run(raw_input, |ctx| {
            egui::TopBottomPanel::top("menubar_container").show(ctx, |ui| {
              egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                  if ui.button("About...").clicked() {
                    println!("Menu option clicked");
                    ui.close_menu();
                  }
                })
              });
            });

            egui::TopBottomPanel::bottom("statusbar_container").show(ctx, |ui| {
              ui.with_layout(
                egui::Layout::left_to_right(egui::Align::Center).with_cross_justify(true),
                |ui| {
                  let mut tape = tape_state.lock().unwrap();

                  if ui.button("Record").clicked() {
                    tape.record = !tape.record;
                  }
                  if ui.button("Play").clicked() {
                    tape.play = !tape.play;
                  }
                  if ui.button("Rewind").clicked() {
                    tape.rewind = !tape.rewind;
                  }
                  if ui.button("Fast Forward").clicked() {
                    tape.fast_forward = !tape.fast_forward;
                  }
                  if ui.button("Stop").clicked() {
                    tape.stop = !tape.stop;
                  }
                  if ui.button("Eject").clicked() {
                    tape.eject = !tape.eject;
                  }

                  ui.label(format!("Recording: {}", tape.record));
                  ui.label(format!("Playing: {}", tape.play));
                  ui.label(format!("Rewinding: {}", tape.rewind));
                  ui.label(format!("Fast Forwarding: {}", tape.fast_forward));
                  ui.label(format!("Stopped: {}", tape.stop));
                  ui.label(format!("Ejected: {}", tape.eject));
                },
              );
            });

            let frame = egui::Frame {
              fill: ctx.style().visuals.window_fill(),
              ..egui::Frame::default()
            };
            egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
              ui.with_layout(
                egui::Layout::centered_and_justified(egui::Direction::TopDown),
                |ui| {
                  ui.image(
                    egui_texture,
                    [
                      (current_config.width as f64 * current_config.scale) as f32,
                      (current_config.height as f64 * current_config.scale) as f32,
                    ],
                  );
                },
              );
            });
          });

          textures.append(output.textures_delta);
          egui_state.handle_platform_output(&window, &egui_ctx, output.platform_output);
          let paint_jobs = egui_ctx.tessellate(output.shapes);

          let render_result = pixels.render_with(|encoder, render_target, context| {
            for (id, image_delta) in &textures.set {
              renderer.update_texture(&context.device, &context.queue, *id, image_delta);
            }
            renderer.update_buffers(
              &context.device,
              &context.queue,
              encoder,
              &paint_jobs,
              &screen_descriptor,
            );

            {
              let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("egui"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                  view: render_target,
                  resolve_target: None,
                  ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                  },
                })],
                depth_stencil_attachment: None,
              });

              renderer.render(&mut rpass, &paint_jobs, &screen_descriptor);
            }

            let textures = std::mem::take(&mut textures);
            for id in &textures.free {
              renderer.free_texture(id);
            }

            Ok(())
          });

          if let Err(e) = render_result {
            eprintln!("Error rendering: {:?}", e);
          }
        }

        Event::WindowEvent { event, .. } => {
          let response = egui_state.on_event(&egui_ctx, &event);

          if response.consumed {
            return;
          }

          match event {
            WindowEvent::KeyboardInput {
              input:
                winit::event::KeyboardInput {
                  virtual_keycode: Some(key),
                  state,
                  ..
                },
              ..
            } => match state {
              ElementState::Pressed => {
                key_state.lock().unwrap().press(key);
              }
              ElementState::Released => {
                key_state.lock().unwrap().release(key);
              }
            },
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            _ => (),
          }
        }
        _ => (),
      }
    });
  }
}

pub struct EguiPlatformProvider {
  config: Arc<Mutex<Option<WindowConfig>>>,
  key_state: Arc<Mutex<KeyState<VirtualKeyCode>>>,
  joystick_state: Arc<Mutex<JoystickState>>,
  tape_state: Arc<Mutex<TapeState>>,
}

impl EguiPlatformProvider {
  pub fn new(
    config: Arc<Mutex<Option<WindowConfig>>>,
    key_state: Arc<Mutex<KeyState<VirtualKeyCode>>>,
    joystick_state: Arc<Mutex<JoystickState>>,
    tape_state: Arc<Mutex<TapeState>>,
  ) -> Self {
    Self {
      config,
      key_state,
      joystick_state,
      tape_state,
    }
  }
}

impl PlatformProvider for EguiPlatformProvider {
  fn request_window(&self, config: WindowConfig) {
    *self.config.lock().unwrap() = Some(config);
  }

  fn get_key_state(&self) -> KeyState<KeyPosition> {
    WinitAdapter::map(&self.key_state.lock().unwrap())
  }

  fn get_virtual_key_state(&self) -> KeyState<VirtualKey> {
    KeyState::new()
  }

  fn get_joystick_state(&self) -> JoystickState {
    *self.joystick_state.lock().unwrap()
  }

  fn get_tape_state(&self) -> TapeState {
    *self.tape_state.lock().unwrap()
  }

  fn print(&self, text: &str) {
    print!("{text}");
  }

  fn input(&self) -> String {
    let mut input = String::new();
    print!("> ");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");
    input
  }

  fn random(&self) -> u8 {
    rand::random()
  }
}
