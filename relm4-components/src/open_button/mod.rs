//! Reusable and easily configurable open button dialog component.
//!
//! **[Example implementation](https://github.com/Relm4/Relm4/blob/main/relm4-components/examples/open_button.rs)**
use relm4::factory::{DynamicIndex, FactoryVecDeque};
use relm4::gtk::prelude::*;
use relm4::{
    gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller,
    SimpleComponent,
};

use crate::open_dialog::{OpenDialog, OpenDialogMsg, OpenDialogResponse, OpenDialogSettings};

use std::fs;
use std::path::PathBuf;

mod factory;

use factory::FileListItem;

/// Open button component.
///
/// Creates a button with custom text that can be used to open a file chooser dialog. If a file is
/// chosen, then it will be emitted as an output. The component can also optionally display a
/// popover list of open files if [`OpenButtonSettings::recently_opened_files`] is set to a value.
#[tracker::track]
#[derive(Debug)]
pub struct OpenButton {
    #[do_not_track]
    config: OpenButtonSettings,
    #[do_not_track]
    dialog: Controller<OpenDialog>,
    #[do_not_track]
    recent_files: Option<FactoryVecDeque<FileListItem>>,
    initialized: bool,
    #[do_not_track]
    reset_popover: bool,
}

#[derive(Debug)]
/// Configuration for the open button component
pub struct OpenButtonSettings {
    /// Settings for the open file dialog.
    pub dialog_settings: OpenDialogSettings,
    /// Icon of the open button.
    pub icon: Option<&'static str>,
    /// Text of the open button.
    pub text: &'static str,
    /// Path to a file where recent files should be stored.
    /// This list is updated fully automatically.
    pub recently_opened_files: Option<&'static str>,
    /// Maximum amount of recent files to store.
    /// This is only used if a path for storing the recently opened files was set.
    pub max_recent_files: usize,
}

#[doc(hidden)]
#[derive(Debug)]
pub enum OpenButtonMsg {
    Open(PathBuf),
    OpenRecent(DynamicIndex),
    ShowDialog,
    Ignore,
}

/// Widgets of the open button component
#[relm4::component(pub)]
impl SimpleComponent for OpenButton {
    type Init = OpenButtonSettings;
    type Input = OpenButtonMsg;
    type Output = PathBuf;

    view! {
        gtk::Box {
            add_css_class: relm4::css::LINKED,
            gtk::Button {
                connect_clicked => OpenButtonMsg::ShowDialog,

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 5,

                    gtk::Image {
                        set_visible: model.config.icon.is_some(),
                        set_icon_name: model.config.icon,
                    },

                    gtk::Label {
                        set_label: model.config.text,
                    }
                }
            },
            gtk::MenuButton {
                #[watch]
                set_visible: model.config.recently_opened_files.is_some()
                    && model.recent_files.is_some(),

                #[wrap(Some)]
                #[name(popover)]
                set_popover = &gtk::Popover {
                    gtk::ScrolledWindow {
                        set_hscrollbar_policy: gtk::PolicyType::Never,
                        set_min_content_width: 100,
                        set_min_content_height: 100,
                        set_min_content_height: 300,

                        #[local_ref]
                        recent_files_list -> gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_vexpand: true,
                            set_hexpand: true,
                        }
                    }
                }
            }
        }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        self.reset_popover = false;

        match msg {
            OpenButtonMsg::ShowDialog => {
                self.dialog.emit(OpenDialogMsg::Open);
            }
            OpenButtonMsg::Open(path) => {
                sender.output(path.clone()).unwrap();
                self.reset_popover = true;

                if let Some(recent_files) = &mut self.recent_files {
                    let index = recent_files.iter().position(|item| item.path == path);

                    if let Some(index) = index {
                        recent_files.guard().remove(index);
                    }

                    if recent_files.len() < self.config.max_recent_files {
                        recent_files.guard().push_front(path);
                    }

                    let contents = recent_files
                        .iter()
                        .filter_map(|recent_path| {
                            recent_path.path.to_str().map(|s| format!("{s}\n"))
                        })
                        .collect::<String>();

                    let _ = fs::write(self.config.recently_opened_files.unwrap(), contents);
                }
            }
            OpenButtonMsg::OpenRecent(index) => {
                if let Some(item) = self
                    .recent_files
                    .as_ref()
                    .and_then(|recent_files| recent_files.get(index.current_index()))
                {
                    sender.input(OpenButtonMsg::Open(PathBuf::from(&item.path)));
                }
            }
            OpenButtonMsg::Ignore => (),
        }
    }

    fn pre_view() {
        if self.reset_popover {
            popover.popdown();
        }
    }

    fn init(
        settings: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let dialog = OpenDialog::builder()
            .transient_for_native(&root)
            .launch(settings.dialog_settings.clone())
            .forward(sender.input_sender(), |response| match response {
                OpenDialogResponse::Accept(path) => OpenButtonMsg::Open(path),
                OpenDialogResponse::Cancel => OpenButtonMsg::Ignore,
            });

        let recent_files_list = gtk::Box::default();

        let mut model = Self {
            config: settings,
            dialog,
            initialized: false,
            recent_files: None,
            reset_popover: false,
            tracker: 0,
        };

        if let Some(filename) = model.config.recently_opened_files {
            let mut factory = FactoryVecDeque::builder()
                .launch(recent_files_list.clone())
                .forward(sender.input_sender(), |msg| msg);

            if let Ok(entries) = fs::read_to_string(filename) {
                let mut guard = factory.guard();
                for entry in entries.lines() {
                    guard.push_back(PathBuf::from(entry));
                }
            }

            model.recent_files = Some(factory);
        }

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
