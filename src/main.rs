use relm4::prelude::*;
// RÄTT IMPORT: Relm4 återexporterar gtk4 som 'gtk'
use relm4::gtk::prelude::*;

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
            set_title: Some("Mitt första Relm4-fönster"),
            set_default_size: (300, 200),
        }
    }
}

// 4. Starta applikationen i main-funktionen
fn main() {
    let app = RelmApp::new("se.exempel.enkelt_fonster");
    app.run::<App>(());
}
