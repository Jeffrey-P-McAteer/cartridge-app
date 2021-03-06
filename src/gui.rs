
use crate::*;

pub fn draw_settings_win() {
  //println!("draw_settings_win commented to save on compile time");
  let mut gui = GuiObject::new();
  while gui.handle_winit_events() != false {
      gui.draw();
  }
}

pub fn make_tray(icon_path: String) {
  println!("icon_path={}", icon_path);
  let mut app;
  match systray::Application::new() {
      Ok(w) => app = w,
      Err(_) => panic!("Can't create window!")
  }
  // w.set_tooltip(&"Whatever".to_string());
  if icon_path.len() > 1 {
    app.set_icon_from_file(&icon_path).ok();
    
    //#[cfg(target_family = "windows")]
    //app.set_icon_from_resource(&icon_path).ok();
    
  }
  app.add_menu_item(&"Cartridge App".to_string(), |_| {
      Ok::<_, systray::Error>(())
  }).ok();
  app.add_menu_separator().ok();
  app.add_menu_item(&"Open Settings".to_string(), |_window| {
      crate::open_settings();
      Ok::<_, systray::Error>(())
  }).ok();
  app.add_menu_item(&"Quit".to_string(), |window| {
      window.quit();
      Ok::<_, systray::Error>(())
  }).ok();
  println!("Beginning event loop...");
  app.wait_for_message();
}


widget_ids! {
    pub struct Ids {
        text, words_input
    }
}

/// events_loop: poll events from windows
/// ui: "where" to display
/// ids: Custom struct that does countain all our widget
/// renderer: Interface between conrod's Primitives && glium's "Surface"
/// image_map should contain all images widgets. None here.
struct GuiObject {
  pub events_loop:    glium::glutin::EventsLoop,
  pub display:        glium::Display,
  pub ui:             conrod::Ui,
  pub ids:            Ids,
  pub renderer:       conrod::backend::glium::Renderer,
  pub image_map:      conrod::image::Map<glium::texture::Texture2d>,
  pub words_input_value: String,
}

impl GuiObject {
    fn new() -> GuiObject {
        const WIDTH: u32 = 400;
        const HEIGHT: u32 = 200;
        let events_loop = glium::glutin::EventsLoop::new();
        let window = glium::glutin::WindowBuilder::new()
            .with_title("Hello Conrod!")
            .with_dimensions((WIDTH, HEIGHT).into());
        let context = glium::glutin::ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(4);
        let display = glium::Display::new(window, context, &events_loop).unwrap();
        let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();
        let ids = Ids::new(ui.widget_id_generator());
        const FONT_PATH: &'static str =
           concat!(env!("CARGO_MANIFEST_DIR"), "/assets/ttf/Hack-Regular.ttf");
        ui.fonts.insert_from_file(FONT_PATH).unwrap();
        let renderer = conrod::backend::glium::Renderer::new(&display).unwrap();
        let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

        GuiObject {
            events_loop:    events_loop,
            display:        display,
            ui:             ui,
            ids:            ids,
            renderer:       renderer,
            image_map:      image_map,
            words_input_value: String::new(),
        }
    }

    fn update(&mut self) {
        let ui = &mut self.ui.set_widgets();

        // add widgets to screen
        conrod::widget::Text::new("Cartridge App Settings")
            .top_left_of(ui.window)
            .color(conrod::color::BLACK)
            .font_size(24)
            .set(self.ids.text, ui);
        
        for edit in conrod::widget::text_box::TextBox::new(&self.words_input_value)
            .mid_left_of(ui.window)
            .color(conrod::color::WHITE)
            .font_size(14)
            .set(self.ids.words_input, ui) {
          match edit {
            conrod::widget::text_box::Event::Enter => {
                
            }
            conrod::widget::text_box::Event::Update(text) => {
                self.words_input_value = format!("{}", text);
            }
          }
        }
        
    }

    /// @return: false if user asked to close windows.
    fn process_event(&mut self, event: conrod::glium::glutin::Event) -> bool {
        match event.clone() {
            glium::glutin::Event::WindowEvent { event, .. } => {
                match event {
                    glium::glutin::WindowEvent::CloseRequested |
                        glium::glutin::WindowEvent::KeyboardInput {
                            input: glium::glutin::KeyboardInput {
                                virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                                ..
                            },
                            ..
                        } => return false,
                    glium::glutin::WindowEvent::ReceivedCharacter('q') => {
                      return false;
                    },
                    // glium::glutin::WindowEvent::KeyboardInput{ input, .. } => {
                    //   if input.state == glium::glutin::ElementState::Released {
                    //     if let Some(glium::glutin::VirtualKeyCode::Delete) = input.virtual_keycode {
                    //       self.words_input_value.pop();
                    //     }
                    //   }
                    // }
                    evt => {
                      println!("evt={:?}", evt);
                      ()
                    },
                }
            }
            _ => (),
        };

        // convert winit events to conrod events
        match conrod::backend::winit::convert_event(event, &self.display) {
            None => return true,
            Some(input) => {
                self.ui.handle_event(input);
                self.update();
            }
        }
        true
    }

    /// Get all the new events since last frame.
    /// If there are none, wait for one.
    /// @return: false if user asked to close windows.
    fn handle_winit_events(&mut self) -> bool {
        let mut events = Vec::new();

        self.events_loop.poll_events(|event| { events.push(event); });
        if events.is_empty() {
            self.events_loop.run_forever(|event| {
                events.push(event);
                glium::glutin::ControlFlow::Break
            });
        }

        for event in events.drain(..) {
            if self.process_event(event) == false {
                return false;
            }
        }
        true
    }

    fn draw(&mut self) {
        if let Some(primitives) = self.ui.draw_if_changed() {
            self.renderer.fill(&self.display, primitives, &self.image_map);
            let mut target = self.display.draw();
            // Icon Folder BG color
            target.clear_color(0.961, 0.808, 0.573, 1.0);
            self.renderer.draw(&self.display, &mut target, &self.image_map).unwrap();
            target.finish().unwrap();
        }
    }
}
