use super::FactoryComponent;

use std::cell::RefCell;
use std::rc::Rc;

use super::FactoryView;
use crate::Sender;

#[derive(Debug)]
pub(super) struct FactoryHandle<Widget, C: FactoryComponent<Widget, ParentMsg>, ParentMsg>
where
    Widget: FactoryView,
    C: FactoryComponent<Widget, ParentMsg>,
{
    pub(super) data: Rc<RefCell<C>>,
    pub(super) root_widget: C::Root,
    pub(super) returned_widget: Widget::ReturnedWidget,
    pub(super) input: Sender<C::Input>,
    pub(super) notifier: Sender<()>,

    /// Kills the event loop of this component on drop.
    pub(super) burner: crate::component::CompBurner,
}
