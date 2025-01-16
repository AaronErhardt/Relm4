use std::path::PathBuf;

use gtk::prelude::*;
use relm4::{
    gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller, RelmApp,
    SimpleComponent,
};
use relm4_components::open_button::{OpenButton, OpenButtonSettings};
use relm4_components::open_dialog::OpenDialogSettings;

#[derive(Debug)]
enum AppMsg {
    Open(PathBuf),
}

struct App {
    open_button: Controller<OpenButton>,
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    view! {
        gtk::ApplicationWindow {
            set_default_size: (300, 100),

            #[wrap(Some)]
            set_titlebar = &gtk::HeaderBar {
                pack_start: model.open_button.widget(),
            }
        }
    }

    fn update(&mut self, msg: Self::Input, _: ComponentSender<Self>) {
        match msg {
            AppMsg::Open(path) => {
                println!("* Opened file {path:?} *");
            }
        }
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let open_button = OpenButton::builder()
            .launch(OpenButtonSettings {
                dialog_settings: OpenDialogSettings::default(),
                icon: None,
                text: "Open file",
                recently_opened_files: Some(".recent_files"),
                max_recent_files: 10,
            })
            .forward(sender.input_sender(), AppMsg::Open);
        let model = App { open_button };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}

fn main() {
    let app = RelmApp::new("relm4.example.open_button");
    app.run::<App>(());
}
