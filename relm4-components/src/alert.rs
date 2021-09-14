//! Reusable and easily configurable alert component.
//!
//! **[Example implementation](https://github.com/AaronErhardt/relm4/blob/main/relm4-examples/examples/alert.rs)**

use gtk::prelude::{DialogExt, GtkWindowExt, WidgetExt};
use relm4::{send, ComponentUpdate, Model, Sender};

use crate::ParentWindow;

#[derive(Clone)]
/// Configuration for the alert dialog component
pub struct AlertSettings {
    /// Large text
    pub text: &'static str,
    /// Optional secondary, smaller text
    pub secondary_text: Option<&'static str>,
    /// Modal dialogs freeze other windows as long they are visible
    pub is_modal: bool,
    /// Sets color of the accept button to red if the theme supports it
    pub destructive_accept: bool,
    /// Text for confirm button
    pub confirm_label: &'static str,
    /// Text for cancel button
    pub cancel_label: &'static str,
    /// Text for third option button. If [`None`] the third button won't be created.
    pub option_label: Option<&'static str>,
}

/// Model of the alert dialog component
pub struct AlertModel {
    settings: AlertSettings,
    is_active: bool,
}

/// Messages that can be sent to the alert dialog component
pub enum AlertMsg {
    /// Message sent by the parent to view the dialog
    Show,
    #[doc(hidden)]
    Response(gtk::ResponseType),
}

impl Model for AlertModel {
    type Msg = AlertMsg;
    type Widgets = AlertWidgets;
    type Components = ();
    type Settings = AlertSettings;
}

/// Interface for the parent model
pub trait AlertParent: Model
where
    Self::Widgets: ParentWindow,
{
    /// Message sent to parent if user clicks confirm button
    fn confirm_msg() -> Self::Msg;

    /// Message sent to parent if user clicks cancel button
    fn cancel_msg() -> Self::Msg;

    /// Message sent to parent if user clicks third option button
    fn option_msg() -> Self::Msg;
}

impl<ParentModel> ComponentUpdate<ParentModel> for AlertModel
where
    ParentModel: AlertParent,
    ParentModel::Widgets: ParentWindow,
{
    fn init_model(_parent_model: &ParentModel, settings: &AlertSettings) -> Self {
        AlertModel {
            settings: settings.clone(),
            is_active: false,
        }
    }

    fn update(
        &mut self,
        msg: AlertMsg,
        _components: &(),
        _sender: Sender<AlertMsg>,
        parent_sender: Sender<ParentModel::Msg>,
    ) {
        match msg {
            AlertMsg::Show => {
                self.is_active = true;
            }
            AlertMsg::Response(ty) => {
                self.is_active = false;
                parent_sender
                    .send(match ty {
                        gtk::ResponseType::Accept => ParentModel::confirm_msg(),
                        gtk::ResponseType::Other(_) => ParentModel::option_msg(),
                        _ => ParentModel::cancel_msg(),
                    })
                    .unwrap();
            }
        }
    }
}

#[relm4_macros::widget(pub)]
/// Widgets of the alert component
impl<ParentModel> relm4::Widgets<AlertModel, ParentModel> for AlertWidgets
where
    ParentModel: AlertParent,
    ParentModel::Widgets: ParentWindow,
{
    view! {
        dialog = gtk::MessageDialog {
            set_transient_for: parent_widgets.parent_window().as_ref(),
            set_message_type: gtk::MessageType::Question,
            set_visible: watch!(model.is_active),
            connect_response(sender) => move |_, response| {
                send!(sender, AlertMsg::Response(response));
            },

            // Apply configuration
            set_text: Some(model.settings.text),
            set_secondary_text: model.settings.secondary_text.as_deref(),
            set_modal: model.settings.is_modal,
            add_button: args!(model.settings.confirm_label, gtk::ResponseType::Accept),
            add_button: args!(model.settings.cancel_label, gtk::ResponseType::Cancel),
        }
    }

    fn post_init() {
        if let Some(option_label) = &model.settings.option_label {
            dialog.add_button(option_label, gtk::ResponseType::Other(0));
        }
        if model.settings.destructive_accept {
            let accept_widget = dialog
                .widget_for_response(gtk::ResponseType::Accept)
                .expect("No button for accept response set");
            accept_widget.add_css_class("destructive-action");
        }
    }
}
