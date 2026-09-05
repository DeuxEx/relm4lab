#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]


use relm4::prelude::*;
use relm4::gtk::prelude::*;

use relm4::gtk::prelude::GtkWindowExt;
use relm4::gtk::prelude::WidgetExt; // Behövs för marginaler och layout



mod dbusreceiver;

// 1. Appens tillstånd (Görs publika så dbusreceiver kan se dem)
pub struct App {
    pub visad_text: String,
}

// 2. Meddelanden för att uppdatera GUI
#[derive(Debug)]
pub enum AppMsg {
    UppdateraText(String),
}


// 3. Implementera komponenten
#[relm4::component(pub)] // <-- Skicka in 'pub' som argument här!
impl SimpleComponent for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = App {
            visad_text: String::from("Väntar på D-Bus meddelande..."),
        };

        // Klonga sendern och starta D-Bus-funktionen asynkront i bakgrunden
        let thread_sender = sender.clone();
        tokio::spawn(async move {
            dbusreceiver::starta_dbus_lyssnare(thread_sender).await;
        });

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::UppdateraText(ny_text) => {
                self.visad_text = ny_text;
            }
        }
    }

    view! {
        gtk::Window {
            set_title: Some("Uppdelad D-Bus i Relm4"),
            set_default_size: (400, 150),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 10,
                set_margin_all: 20,
                set_valign: gtk::Align::Center,

                gtk::Label {
                    set_label: "Mottaget via D-Bus:",
                    add_css_class: "caption",
                },

                gtk::Label {
                    #[watch]
                    set_label: &model.visad_text,
                    add_css_class: "title-2",
                }
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("se.exempel.dbus_mottagare");
    app.run::<App>(());
}
