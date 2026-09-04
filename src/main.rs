
use relm4::prelude::*;

// RÄTT IMPORT: Relm4 återexporterar gtk4 som 'gtk'
use relm4::gtk::prelude::*;


mod dbusreceiver;
use dbusreceiver::{main as other_main};


// 1. Skapa en struktur för applikationens tillstånd (State)
struct App;

// 2. Definiera vilka meddelanden (Events) som appen ska kunna ta emot
#[derive(Debug)]
enum AppMsg {}

// 3. Implementera komponenten med Relm4:s Component-trait
#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    // Skapar starttillståndet för appen
    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = App;

        // Bygger upp fönstret via det interna makrot
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    // Bestämmer vad som händer när ett meddelande tas emot
    fn update(&mut self, _msg: Self::Input, _sender: ComponentSender<Self>) {}

    // GUI-strukturen skrivs med ett deklarativt makro
    view! {
        // Vi använder 'gtk' som kommer från den importerade preluden ovan
        gtk::Window {
            set_title: Some("Mitt första Relm4-fönster"),   // Sätter fönstertiteln
            set_default_size: (800, 600),                   // Bredd och höjd vid start
            set_resizable: true,                           // Hindrar användaren från att ändra storlek
            //set_maximized: true,                          // Startar fönstret i helskärm
            //set_decorations: false,                       // Tar bort systemramen/kryssknappen (borderless)
            set_opacity: 0.9,                               // Gör fönstret en aning transparent

            // HUVUDBOX (Motsvarar en vertikal StackPanel)
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 12,          // Avstånd mellan elementen (i pixlar)
                set_margin_all: 15,       // "Padding" runt hela innerinnehållet

                // 1. En rubrik (Label)
                gtk::Label {
                    set_label: "Välkommen till Relm4",
                    // Vi kan styla texten med inbyggda CSS-klasser i GTK
                    add_css_class: "title-1",
                },

                // 2. Ett textinmatningsfält (Motsvarar TextBox)
                gtk::Entry {
                    set_placeholder_text: Some("Skriv något här..."),
                    // Gör så att textfältet suger åt sig allt ledigt vertikalt utrymme
                    set_vexpand: true,
                    set_valign: gtk::Align::Center,
                },

                // 3. RAD MED KNAPPAR (Motsvarar en horisontell StackPanel längst ner)
                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 6,
                    // Knuffar hela denna box till höger sida
                    set_halign: gtk::Align::End,

                    gtk::Button {
                        set_label: "Avbryt",
                    },

                    gtk::Button {
                        set_label: "OK",
                        // Gör knappen blå/accentfärgad (GTK standard-styling)
                        add_css_class: "suggested-action",
                    },
                }
            }




        }
    }
}

// 4. Starta applikationen i main-funktionen
fn main() {
    let app = RelmApp::new("se.exempel.enkelt_fonster");
    let _ = dbusreceiver::main();
    app.run::<App>(());
}
