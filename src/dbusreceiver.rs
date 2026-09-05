#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]


use relm4::ComponentSender;
use zbus::{interface, connection};
use crate::{App, AppMsg};

pub struct DbusReceiver {
    gui_sender: ComponentSender<App>,
}

#[interface(name = "se.exempel.GuiInterface")]
impl DbusReceiver {
    async fn skicka_text(&self, text: String) {
        let _ = self.gui_sender.input(AppMsg::UppdateraText(text));
    }
}

pub async fn starta_dbus_lyssnare(sender: ComponentSender<App>) {
    let receiver = DbusReceiver { gui_sender: sender };

    // Vi sparar anslutningen i en variabel 'conn'
    //let conn = connection::Builder::system() // <--- Ändrat från .session() för att klara både vanilj och root, kräver att man skapar en policy
    let conn = connection::Builder::session() // <--- Ändrat från .system() för att vanilj user ska fungera
    .unwrap()
    .name("se.exempel.GuiService").unwrap()
    .serve_at("/se/exempel/GuiObject", receiver).unwrap()
    .build()
    .await;

    match conn {
        Ok(_connection) => {
            println!("🚀 D-Bus-tjänsten startade framgångsrikt som 'se.exempel.GuiService'!");

            // VIKTIGT: Vi flyttar in pending HÄR så att '_connection' inte droppas ur minnet!
            std::future::pending::<()>().await;
        }
        Err(e) => {
            eprintln!("❌ D-Bus misslyckades att starta: {:?}", e);
        }
    }
}
