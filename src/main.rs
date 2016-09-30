extern crate ws;
#[macro_use] extern crate conrod;
extern crate find_folder;
extern crate piston_window;

use std::thread;

use ws::listen;
use ws::{connect, CloseCode};

use piston_window::{EventLoop, OpenGL, PistonWindow, UpdateEvent, WindowSettings};

fn main()
{
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;
    
    let opengl = OpenGL::V3_2;
    
    // Construct the window.
    let mut window: PistonWindow =
        WindowSettings::new("Socketmessenger", [WIDTH, HEIGHT])
            .opengl(opengl).exit_on_esc(true).build().unwrap();
    window.set_ups(60);
    
    

    // construct our `Ui`.
    let mut ui = conrod::UiBuilder::new().build();
    
    // Add a `Font` to the `Ui`'s `font::Map` from file.
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();
    
    
    // Create a texture to use for efficiently caching text on the GPU.
    let mut text_texture_cache = conrod::backend::piston_window::GlyphCache::new(&mut window, WIDTH, HEIGHT);
    
    
    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod::image::Map::new();
    
    
    // Instantiate the generated list of widget identifiers.
    let ids = &mut Ids::new(ui.widget_id_generator());
    
    let mut textedit_text = "Test".to_owned();
    let mut chat_text = "Test2".to_owned();
    
    
    // Poll events from the window.
    while let Some(event) = window.next()
    {
        // Convert the piston event to a conrod event.
        if let Some(e) = conrod::backend::piston_window::convert_event(event.clone(), &window) {
            ui.handle_event(e);
        }

        event.update(|_| {
            set_widgets(ui.set_widgets(), ids, &mut textedit_text, &mut chat_text);
        });

        window.draw_2d(&event, |c, g| {
            if let Some(primitives) = ui.draw_if_changed() {
                fn texture_from_image<T>(img: &T) -> &T { img };
                conrod::backend::piston_window::draw(c, g, primitives,
                                                     &mut text_texture_cache,
                                                     &image_map,
                                                     texture_from_image);
            }
        });
    }
/*
*This is the server-part of the chat-program, currently simply echoes back what was recieved.
*It is here that chat messages should be presented to the user.
**/
    thread::spawn(move || {
        listen
        ("127.0.0.1:3012",
            |out| {
            move |msg| {
                out.send(&*format!("{0}{1}{0}",msg," SVAR! "))
                }
            }
        ).unwrap(); 
    });

/*
*This is the client-part of the chat-program, currently simply sends one message and then ends the program.
*It is here that input should be placed into the program.
*Should I have this in a thread seperate from the main thread as well or simply have a main loop in the main-thread?
**/
    std::thread::sleep(std::time::Duration::from_millis(1000));
    connect("ws://127.0.0.1:3012", |out| {
        out.send("Hello WebSocket").unwrap();

        move |msg| {
            println!("Got message: {}", msg);
            out.close(CloseCode::Normal)
        }
    }).unwrap()
}




// Draw the Ui.
fn set_widgets(ref mut ui: conrod::UiCell, ids: &mut Ids, textedit_text: &mut String, chat_text: &mut String) {
    use conrod::{color, widget, Colorable, Positionable, Scalar, Sizeable, Widget};
    
    widget::Canvas::new().flow_down(&[
        (ids.header, widget::Canvas::new().color(color::BLUE).pad_bottom(20.0)),
        (ids.body, widget::Canvas::new().flow_right(&[
            (ids.send_text, widget::Canvas::new().color(color::RED).pad_bottom(20.0)),
            (ids.send_button, widget::Canvas::new().color(color::BLACK).pad_bottom(20.0))
            ]))
        ]).set(ids.master, ui);
        

    const PAD: Scalar = 20.0;
    /*
    *Add a TextBox to hold all chat send/recieved.
    **/
    
    widget::Text::new(chat_text)
        .color(color::LIGHT_RED)
        .top_left_with_margins_on(ids.header, 15.0, 15.0)
        .align_text_left()
        .font_size(15)
        .line_spacing(10.0)
        .set(ids.chat_text, ui);

    for edit in widget::TextEdit::new(textedit_text)
        .top_left_with_margins_on(ids.send_text, 15.0, 15.0)
        .font_size(15)
        .padded_w_of(ids.send_text, PAD)
        .color(conrod::color::rgb(20.2, 40.35, 0.45))
        .set(ids.textedit, ui)
        {
            *textedit_text = edit;
        };

    /*
    *Change the button to handle enter and click event to send.
    **/
    let button = widget::Button::new().color(color::RED).w_h(30.0, 30.0);
    for _click in button.clone().middle_of(ids.floating_a).set(ids.bing, ui) {
        println!("Bing!");
    }
    for _click in button.middle_of(ids.floating_b).set(ids.bong, ui) {
        println!("Bong!");
    }
}


// Generate a unique `WidgetId` for each widget.
widget_ids! {
    struct Ids {
        master,
        header,
        chat_text,
        body,
        send_text,
        send_button,
        floating_a,
        floating_b,
        bing,
        bong,
        textedit
    }
}
