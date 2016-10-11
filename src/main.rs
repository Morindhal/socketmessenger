extern crate ws;
#[macro_use] extern crate conrod;
extern crate find_folder;
extern crate piston_window;

use std::thread;

use ws::listen;
use ws::{connect, CloseCode};

use piston_window::{EventLoop, OpenGL, PistonWindow, UpdateEvent};

fn main()
{

    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;
    
    let opengl = OpenGL::V3_2;
    
    // Construct the window.
    let mut window: PistonWindow =
        piston_window::WindowSettings::new("Socketmessenger", [WIDTH, HEIGHT])
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
    
    let mut textedit_text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
        Mauris aliquet porttitor tellus vel euismod. Integer lobortis volutpat bibendum. Nulla \
        finibus odio nec elit condimentum, rhoncus fermentum purus lacinia. Interdum et malesuada \
        fames ac ante ipsum primis in faucibus. Cras rhoncus nisi nec dolor bibendum pellentesque. \
        Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. \
        Quisque commodo nibh hendrerit nunc sollicitudin sodales. Cras vitae tempus ipsum. Nam \
        magna est, efficitur suscipit dolor eu, consectetur consectetur urna.".to_owned();
    let mut chat_text = "Test2".to_owned();

/*
*This is the server-part of the chat-program, currently simply echoes back what was recieved.
*It is here that chat messages should be presented to the user.
**/
    let server_listen = thread::spawn(move || {
        listen
        ("127.0.0.1:3012",
            |out| {
            move |msg| {
                out.send(&*format!("{0}{1}{0}",msg, " SVAR! "))
                //&*format!("{0}{1}{0}",msg, " SVAR! ")
                }
            }
        ).unwrap(); 
    });
    
/*
*This is the main loop of the program.
**/
    // Poll events from the window.
    while let Some(event) = window.next()
    {
        // Convert the piston event to a conrod event.
        if let Some(e) = conrod::backend::piston_window::convert_event(event.clone(), &window) {
            ui.handle_event(e);
        }

        event.update(|_| set_widgets(ui.set_widgets(), ids, &mut textedit_text, &mut chat_text));

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
}




// Draw the Ui.
fn set_widgets(ref mut ui: conrod::UiCell, ids: &mut Ids, textedit_text: &mut String, chat_text: &mut String) {
    use conrod::{color, widget, Colorable, Positionable, Scalar, Sizeable, Widget};
    
    widget::Canvas::new().flow_down(&[
        (ids.header, widget::Canvas::new().color(color::BLUE).pad_bottom(20.0)),
        (ids.body, widget::Canvas::new().flow_right(&[
            (ids.send_text, widget::Canvas::new().color(color::RED).crop_kids().scroll_kids()),
            (ids.send_button, widget::Canvas::new().color(color::BLACK).length_weight(0.1))
            ]).length_weight(0.5))
        ]).set(ids.master, ui);
        

    const PAD: Scalar = 20.0;
    
    widget::Tabs::new(&[(ids.tab_chattext1, "Chat 1"), (ids.tab_chattext2, "Chat 2")])
        .wh_of(ids.header)
        .color(color::BLUE)
        .label_color(color::WHITE)
        .middle_of(ids.header)
        .set(ids.tabs, ui);

//This is the text that should echo all recieved chat, as soon as I figure out how to do it :)
//I really need to read up more on threads in rust.
    widget::Text::new(chat_text)
        .color(color::LIGHT_RED)
        .top_left_with_margins_on(ids.tab_chattext1, 15.0, 15.0)
        .padded_w_of(ids.tab_chattext1, PAD)
        .align_text_left()
        .font_size(15)
        .line_spacing(10.0)
        .set(ids.chat_text, ui);

//This is the user-editable chat text.
    for edit in widget::TextEdit::new(textedit_text)
        .parent(ids.send_text)
        .mid_top_of(ids.send_text)
        .align_text_left()
        .font_size(12)
        .padded_w_of(ids.send_text, PAD)
        .h(400.0)
        .color(conrod::color::rgb(20.2, 40.35, 0.45))
        .restrict_to_height(true)
        .set(ids.textedit, ui)
        {
            *textedit_text = edit;
        }
    widget::Scrollbar::y_axis(ids.send_text)
        .auto_hide(true)
        .color(color::GRAY)
        .set(ids.chat_text_scrollbar, ui);

//This is the send button that sends whats in the TextEdit and then erases it.
    let button = widget::Button::new().color(color::GRAY).w_h(30.0, 30.0);
    for _click in button.clone().middle_of(ids.send_button).set(ids.bing, ui) {

/*
*This is the client-part of the chat-program, currently simply sends one message and println's the response as well as what was sent.
**/
            connect("ws://127.0.0.1:3012", |out| {
                out.send(&*format!("{}",textedit_text)).unwrap();

                move |msg| {
//                    println!("Got response: {}", msg);
                    out.close(CloseCode::Normal)
                }
            }).unwrap();
        chat_text.push_str(&*format!("\n{}", textedit_text));
        *textedit_text = String::new();
    }
}


// Generate a unique `WidgetId` for each widget.
widget_ids! {
    struct Ids {
        master,
        header,
        chat_text_scrollbar,
        chat_text,
        body,
        tabs,
        tab_chattext1,
        tab_chattext2,
        tab_sendbutton,
        send_text,
        send_button,
        floating_a,
        floating_b,
        bing,
        bong,
        textedit
    }
}
