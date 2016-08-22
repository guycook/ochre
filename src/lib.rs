extern crate piston_window;
extern crate gfx_device_gl;
#[macro_use]
extern crate conrod;

use conrod::{widget, Labelable, Positionable, Sizeable, Widget};
use piston_window::{EventLoop, PistonWindow, UpdateEvent, WindowSettings};

// TODO: Because the ochre_app macro is called from user code, nearly everything in this module
//       has to be public. Find some way around this.

pub trait New {
    fn new() -> Self;
}

pub enum Component {
    Button,
}

#[macro_export]
macro_rules! ochre {
    // For now, the top level declaration must be a Window, optionally with a backing data struct
    // TODO: ^^ Will this always be the case?
    (Window { $($body:tt)* }) => ({
        println!("{}", concat!($(stringify!($body)," - "),*));
        ochre_component!($($body)*);
    });
    (Window<$data:ty> { $($body:tt)* }) => ({
        println!("{}", concat!($(stringify!($body)," - "),*));
        ochre_component!($($body)*);
    });
}

// TODO: Find out if there's some way to do `my_macro!(include!("file"))`. So far everything I've
//       tried just passes in the include as a tt or expr but doesn't actually execute it.
#[macro_export]
macro_rules! ochre_app {
    ($root:ident) => {
        struct $root {
            window: $crate::Window,
            children: Vec<$crate::Component>,
        }
        impl $root {
            pub fn new() -> $root {
                $root {
                    window: $crate::Window::new(),
                    children: Vec::new(),
                }
            }
            pub fn run(&mut self) {
                // TODO: Can we make $root lower case as a &'static str?
                include!(concat!(stringify!($root), ".ore"));
                $crate::run(&mut self.window);
            }
        }
    }
}

#[macro_export]
macro_rules! ochre_component {
    () => {
        println!("Empty");
    };
    ($k:ident: $v:expr, $($rest:tt)*) => {
        println!("Rule: {}: {}", stringify!($k), stringify!($v));
        println!("{}", concat!($(stringify!($rest)," - "),*));
        ochre_component!($($rest)*);
    };
    ($c:ident { $($body:tt)* } $($rest:tt)*) => {
        println!("{}", concat!($(stringify!($body)," * "),*));
        ochre_component!($($body)*);
        ochre_component!($($rest)*);
    };
    // TODO: A nice solution for nested data
    // ($c:ident<$t:ty> { $($body:tt)* } $($rest:tt)*) => {
    //     println!("{}", concat!($(stringify!($body)," * "),*));
    //     ochre_component!($($body)*);
    //     ochre_component!($($rest)*);
    // };
}

// pub fn app() -> App {
//     let root = o_component!(include!("counter.ore"));
//     App { root: root }
// }

/// TODO: 2nd param is where eg CounterData goes
fn update_ui(ui: &mut conrod::UiCell, count: &mut i32) {
    // Generate the ID for the Button COUNTER.
    widget_ids!(CANVAS, COUNTER);

    // Create a background canvas upon which we'll place the button.
    widget::Canvas::new().pad(40.0).set(CANVAS, ui);

    // Draw the button and increment `count` if pressed.
    if widget::Button::new()
        .middle_of(CANVAS)
        .w_h(80.0, 80.0)
        .label(&count.to_string())
        .set(COUNTER, ui)
        .was_clicked() {
            *count += 1
        }
}

struct ConrodBase {
    ui: conrod::Ui,
    text_texture_cache: conrod::backend::piston_window::GlyphCache,
    image_map: conrod::image::Map<piston_window::Texture<gfx_device_gl::Resources>>,
}

impl ConrodBase {
    pub fn new(window: &mut Window) -> ConrodBase {
        let mut ui = conrod::UiBuilder::new().build();

        // TODO: Choose a default font, perhaps find system UI deafult?
        ui.fonts.insert_from_file("assets/fonts/NotoSans/NotoSans-Regular.ttf").unwrap();

        // Create a texture to use for efficiently caching text on the GPU.
        // TODO: What should these dimensions really be?
        // TODO: Make an issue/PR for a constructor that takes factory
        //       directly so as not to require window
        let text_texture_cache =
            conrod::backend::piston_window::GlyphCache::new(&mut window.window, 300, 200);

        // The image map describing each of our widget->image mappings (in our case, none).
        let image_map = conrod::image::Map::new();

        ConrodBase {
            ui: ui,
            text_texture_cache: text_texture_cache,
            image_map: image_map,
        }
    }

    // TODO: Is it one event loop per window, or per app?
    pub fn run(&mut self, window: &mut Window) {
        let mut count = 0;

        while let Some(event) = window.window.next() {
            // Convert the piston event to a conrod event.
            if let Some(e) = conrod::backend::piston_window::convert_event(event.clone(),
                                                                           &window.window) {
                self.ui.handle_event(e);
            }

            // `Update` the widgets.
            event.update(|_| {
                let mut ui = self.ui.set_widgets();

                update_ui(&mut ui, &mut count);
            });

            // Draw the `Ui` if it has changed.
            window.window.draw_2d(&event, |c, g| {
                if let Some(primitives) = self.ui.draw_if_changed() {
                    conrod::backend::piston_window::draw(c,
                                                         g,
                                                         primitives,
                                                         &mut self.text_texture_cache,
                                                         &self.image_map,
                                                         |img| img);
                }
            });
        }
    }
}

pub struct Window {
    window: PistonWindow,
}

impl Window {
    pub fn new() -> Window {
        let mut window: PistonWindow = WindowSettings::new("Running", [300, 200])
            .opengl(piston_window::OpenGL::V3_2)
            .exit_on_esc(false)
            .build()
            .unwrap();
        window.set_ups(60); // TODO: Is there a builder method for this?

        Window { window: window }
    }
}

pub fn run(window: &mut Window) {
    // TODO: Should this really need window?
    let mut conrod = ConrodBase::new(window);

    conrod.run(window);
}
