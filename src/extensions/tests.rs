use crate::{gtk, RelmBoxExt, RelmFlowBoxExt, RelmGridExt, RelmListBoxExt};
use gtk::prelude::{BoxExt, GridExt, WidgetExt};

// A set of widgets for testing
#[derive(Default)]
struct TestWidgets(gtk::Label, gtk::Switch, gtk::Box);

impl TestWidgets {
    fn assert_parent(&self) {
        assert!(self.0.parent().is_some());
        assert!(self.1.parent().is_some());
        assert!(self.2.parent().is_some());
    }
}

#[gtk::test]
fn box_ext() {
    let gtk_box = gtk::Box::default();
    let widgets = TestWidgets::default();

    gtk_box.append(&widgets.0);
    gtk_box.append(&widgets.1);
    gtk_box.append(&widgets.2);

    widgets.assert_parent();

    let children = gtk_box.children();
    assert_eq!(children.get(0), Some(widgets.0.as_ref()));
    assert_eq!(children.get(1), Some(widgets.1.as_ref()));
    assert_eq!(children.get(2), Some(widgets.2.as_ref()));

    gtk_box.remove_all();

    assert_eq!(gtk_box.children().len(), 0);
}

#[gtk::test]
fn list_box_ext() {
    let list_box = gtk::ListBox::default();
    let widgets = TestWidgets::default();

    list_box.append(&widgets.0);
    list_box.append(&widgets.1);
    list_box.append(&widgets.2);

    widgets.assert_parent();

    assert_eq!(list_box.index_of_child(&widgets.0), Some(0));
    assert_eq!(list_box.index_of_child(&widgets.1), Some(1));
    assert_eq!(list_box.index_of_child(&widgets.2), Some(2));

    let rows = list_box.rows();
    assert_eq!(
        rows.get(0).map(|row| row.as_ref()),
        widgets.0.parent().as_ref()
    );
    assert_eq!(
        rows.get(1).map(|row| row.as_ref()),
        widgets.1.parent().as_ref()
    );
    assert_eq!(
        rows.get(2).map(|row| row.as_ref()),
        widgets.2.parent().as_ref()
    );

    list_box.remove_all();

    assert_eq!(list_box.rows().len(), 0);

    list_box.append(&widgets.0);
    list_box.append(&widgets.1);
    list_box.append(&widgets.2);

    widgets.assert_parent();

    list_box.remove_row_of_child(&widgets.0);
    list_box.remove_row_of_child(&widgets.1);
    list_box.remove_row_of_child(&widgets.2);

    assert_eq!(list_box.rows().len(), 0);

    assert_eq!(list_box.index_of_child(&widgets.0), None);
    assert_eq!(list_box.index_of_child(&widgets.1), None);
    assert_eq!(list_box.index_of_child(&widgets.2), None);
}

#[gtk::test]
fn flow_box_ext() {
    let flow_box = gtk::FlowBox::default();
    let widgets = TestWidgets::default();

    flow_box.insert(&widgets.0, -1);
    flow_box.insert(&widgets.1, -1);
    flow_box.insert(&widgets.2, -1);

    widgets.assert_parent();

    let flow_children = flow_box.flow_children();
    assert_eq!(
        flow_children.get(0).map(|child| child.as_ref()),
        widgets.0.parent().as_ref()
    );
    assert_eq!(
        flow_children.get(1).map(|child| child.as_ref()),
        widgets.1.parent().as_ref()
    );
    assert_eq!(
        flow_children.get(2).map(|child| child.as_ref()),
        widgets.2.parent().as_ref()
    );

    flow_box.remove_all();

    assert_eq!(flow_box.flow_children().len(), 0);
}

#[gtk::test]
fn grid_ext() {
    let grid = gtk::Grid::default();
    let widgets = TestWidgets::default();

    grid.attach(&widgets.0, 0, 0, 1, 1);
    grid.attach(&widgets.1, 1, 0, 1, 1);
    grid.attach(&widgets.2, 2, 2, 2, 2);

    widgets.assert_parent();

    let children = grid.children();
    assert_eq!(children.get(0), Some(widgets.0.as_ref()));
    assert_eq!(children.get(1), Some(widgets.1.as_ref()));
    assert_eq!(children.get(2), Some(widgets.2.as_ref()));

    grid.remove_all();

    assert!(widgets.0.parent().is_none());
    assert!(widgets.1.parent().is_none());
    assert!(widgets.2.parent().is_none());
}
