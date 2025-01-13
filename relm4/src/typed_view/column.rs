//! Idiomatic and high-level abstraction over [`gtk::ColumnView`].

use super::{get_mut_value, get_value, Filter, OrdFn, RelmSelectionExt, TypedListItem};
use gtk::{
    gio, glib,
    prelude::{Cast, CastNone, FilterExt, IsA, ListItemExt, ListModelExt, ObjectExt},
};
use std::{
    any::Any,
    cmp::Ordering,
    collections::HashMap,
    fmt::{Debug, Display},
    marker::PhantomData,
};

/// An item of a [`TypedColumnView`].
pub trait RelmColumn: Any {
    /// The top-level widget for the list item.
    type Root: IsA<gtk::Widget>;

    /// The widgets created for the list item.
    type Widgets;

    /// Item whose data is shown in this column.
    type Item: Any;

    /// The columns created for this list item.
    const COLUMN_NAME: &'static str;
    /// Whether to enable resizing for this column
    const ENABLE_RESIZE: bool = false;
    /// Whether to enable automatic expanding for this column
    const ENABLE_EXPAND: bool = false;

    /// Returns the shown title for the column. By default shows [`RelmColumn::COLUMN_NAME`]. Useful for translations
    #[must_use]
    fn header_title() -> String {
        String::from(Self::COLUMN_NAME)
    }

    /// Construct the widgets.
    fn setup(list_item: &gtk::ListItem) -> (Self::Root, Self::Widgets);

    /// Bind the widgets to match the data of the list item.
    fn bind(_item: &mut Self::Item, _widgets: &mut Self::Widgets, _root: &mut Self::Root) {}

    /// Undo the steps of [`RelmColumn::bind()`] if necessary.
    fn unbind(_item: &mut Self::Item, _widgets: &mut Self::Widgets, _root: &mut Self::Root) {}

    /// Undo the steps of [`RelmColumn::setup()`] if necessary.
    fn teardown(_list_item: &gtk::ListItem) {}

    /// Sorter for column.
    #[must_use]
    fn sort_fn() -> OrdFn<Self::Item> {
        None
    }
}

/// Simplified trait for creating columns with only one `gtk::Label` widget per-entry (i.e. a text cell)
pub trait LabelColumn: 'static {
    /// Item of the model
    type Item: Any;
    /// Value of the column
    type Value: PartialOrd + Display;

    /// Name of the column
    const COLUMN_NAME: &'static str;
    /// Whether to enable the sorting for this column
    const ENABLE_SORT: bool;
    /// Whether to enable resizing for this column
    const ENABLE_RESIZE: bool = false;
    /// Whether to enable automatic expanding for this column
    const ENABLE_EXPAND: bool = false;

    /// Get the value that this column represents.
    fn get_cell_value(item: &Self::Item) -> Self::Value;
    /// Format the value for presentation in the text cell.
    fn format_cell_value(value: &Self::Value) -> String {
        value.to_string()
    }
}

impl<C> RelmColumn for C
where
    C: LabelColumn,
{
    type Root = gtk::Label;
    type Widgets = ();
    type Item = C::Item;

    const COLUMN_NAME: &'static str = C::COLUMN_NAME;
    const ENABLE_RESIZE: bool = C::ENABLE_RESIZE;
    const ENABLE_EXPAND: bool = C::ENABLE_EXPAND;

    fn setup(_: &gtk::ListItem) -> (Self::Root, Self::Widgets) {
        (gtk::Label::new(None), ())
    }

    fn bind(item: &mut Self::Item, _: &mut Self::Widgets, label: &mut Self::Root) {
        label.set_label(&C::format_cell_value(&C::get_cell_value(item)));
    }

    fn sort_fn() -> OrdFn<Self::Item> {
        if C::ENABLE_SORT {
            Some(Box::new(|a, b| {
                let a = C::get_cell_value(a);
                let b = C::get_cell_value(b);
                a.partial_cmp(&b).unwrap_or(Ordering::Equal)
            }))
        } else {
            None
        }
    }
}

/// A high-level wrapper around [`gio::ListStore`],
/// [`gtk::SignalListItemFactory`] and [`gtk::ColumnView`].
///
/// [`TypedColumnView`] aims at keeping nearly the same functionality and
/// flexibility of the raw bindings while introducing a more idiomatic
/// and type-safe interface.
pub struct TypedColumnView<T, S> {
    /// The internal list view.
    pub view: gtk::ColumnView,
    /// The internal selection model.
    pub selection_model: S,
    columns: HashMap<&'static str, gtk::ColumnViewColumn>,
    store: gio::ListStore,
    filters: Vec<Filter>,
    active_model: gio::ListModel,
    base_model: gio::ListModel,
    _ty: PhantomData<*const T>,
}

impl<T, S> Debug for TypedColumnView<T, S>
where
    T: Debug,
    S: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypedColumnView")
            .field("store", &self.store)
            .field("view", &self.view)
            .field("filters", &"<Vec<gtk::Filter>>")
            .field("active_model", &self.active_model)
            .field("base_model", &self.base_model)
            .field("selection_model", &self.selection_model)
            .finish()
    }
}

impl<T, S> Default for TypedColumnView<T, S>
where
    T: Any,
    S: RelmSelectionExt,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, S> TypedColumnView<T, S>
where
    T: Any,
    S: RelmSelectionExt,
{
    /// Create a new, empty [`TypedColumnView`].
    #[must_use]
    pub fn new() -> Self {
        let store = gio::ListStore::new::<glib::BoxedAnyObject>();

        let model: gio::ListModel = store.clone().upcast();

        let b = gtk::SortListModel::new(Some(model), None::<gtk::Sorter>);

        let base_model: gio::ListModel = b.clone().upcast();

        let selection_model = S::new_model(base_model.clone());
        let view = gtk::ColumnView::new(Some(selection_model.clone()));
        b.set_sorter(view.sorter().as_ref());

        Self {
            store,
            view,
            columns: HashMap::new(),
            filters: Vec::new(),
            active_model: base_model.clone(),
            base_model,
            _ty: PhantomData,
            selection_model,
        }
    }

    /// Append column to this typed view
    pub fn append_column<C>(&mut self)
    where
        C: RelmColumn<Item = T>,
    {
        let factory = gtk::SignalListItemFactory::new();
        factory.connect_setup(move |_, list_item| {
            let list_item = list_item
                .downcast_ref::<gtk::ListItem>()
                .expect("Needs to be ListItem");

            let (root, widgets) = C::setup(list_item);
            unsafe { root.set_data("widgets", widgets) };
            list_item.set_child(Some(&root));
        });

        #[inline]
        fn modify_widgets<T, C>(
            list_item: &glib::Object,
            f: impl FnOnce(&mut T, &mut C::Widgets, &mut C::Root),
        ) where
            T: Any,
            C: RelmColumn<Item = T>,
        {
            let list_item = list_item
                .downcast_ref::<gtk::ListItem>()
                .expect("Needs to be ListItem");

            let widget = list_item.child();

            let obj = list_item.item().unwrap();
            let mut obj = get_mut_value::<T>(&obj);

            let mut root = widget.and_downcast::<C::Root>().unwrap();

            let mut widgets = unsafe { root.steal_data("widgets") }.unwrap();
            (f)(&mut *obj, &mut widgets, &mut root);
            unsafe { root.set_data("widgets", widgets) };
        }

        factory.connect_bind(move |_, list_item| {
            modify_widgets::<T, C>(list_item.upcast_ref(), |obj, widgets, root| {
                C::bind(obj, widgets, root);
            });
        });

        factory.connect_unbind(move |_, list_item| {
            modify_widgets::<T, C>(list_item.upcast_ref(), |obj, widgets, root| {
                C::unbind(obj, widgets, root);
            });
        });

        factory.connect_teardown(move |_, list_item| {
            let list_item = list_item
                .downcast_ref::<gtk::ListItem>()
                .expect("Needs to be ListItem");

            C::teardown(list_item);
        });

        let sort_fn = C::sort_fn();

        let c = gtk::ColumnViewColumn::new(Some(&C::header_title()), Some(factory));
        c.set_resizable(C::ENABLE_RESIZE);
        c.set_expand(C::ENABLE_EXPAND);

        if let Some(sort_fn) = sort_fn {
            c.set_sorter(Some(&gtk::CustomSorter::new(move |first, second| {
                let first = get_value::<T>(first);
                let second = get_value::<T>(second);

                sort_fn(&first, &second).into()
            })))
        }

        self.view.append_column(&c);
        self.columns.insert(C::COLUMN_NAME, c);
    }

    /// Add a function to filter the stored items.
    /// Returning `false` will simply hide the item.
    ///
    /// Note that several filters can be added on top of each other.
    pub fn add_filter<F: Fn(&T) -> bool + 'static>(&mut self, f: F) {
        let filter = gtk::CustomFilter::new(move |obj| {
            let value = get_value::<T>(obj);
            f(&value)
        });
        let filter_model =
            gtk::FilterListModel::new(Some(self.active_model.clone()), Some(filter.clone()));
        self.active_model = filter_model.clone().upcast();
        self.selection_model.set_list_model(&self.active_model);
        self.filters.push(Filter {
            filter,
            model: filter_model,
        });
    }

    /// Get columns currently associated with this view.
    pub fn get_columns(&self) -> &HashMap<&'static str, gtk::ColumnViewColumn> {
        &self.columns
    }

    /// Returns the amount of filters that were added.
    pub fn filters_len(&self) -> usize {
        self.filters.len()
    }

    /// Set a certain filter as active or inactive.
    pub fn set_filter_status(&mut self, idx: usize, active: bool) -> bool {
        if let Some(filter) = self.filters.get(idx) {
            if active {
                filter.model.set_filter(Some(&filter.filter));
            } else {
                filter.model.set_filter(None::<&gtk::CustomFilter>);
            }
            true
        } else {
            false
        }
    }

    /// Notify that a certain filter has changed.
    /// This causes the filter expression to be re-evaluated.
    ///
    /// Returns true if a filter was notified.
    pub fn notify_filter_changed(&self, idx: usize) -> bool {
        if let Some(filter) = self.filters.get(idx) {
            filter.filter.changed(gtk::FilterChange::Different);
            true
        } else {
            false
        }
    }

    /// Remove the last filter.
    pub fn pop_filter(&mut self) {
        let filter = self.filters.pop();
        if let Some(filter) = filter {
            self.active_model = filter.model.model().unwrap();
            self.selection_model.set_list_model(&self.active_model);
        }
    }

    /// Remove all filters.
    pub fn clear_filters(&mut self) {
        self.filters.clear();
        self.active_model = self.base_model.clone();
        self.selection_model.set_list_model(&self.active_model);
    }

    /// Add a new item at the end of the list.
    pub fn append(&mut self, value: T) {
        self.store.append(&glib::BoxedAnyObject::new(value));
    }

    /// Add new items from an iterator the the end of the list.
    pub fn extend_from_iter<I: IntoIterator<Item = T>>(&mut self, init: I) {
        let objects: Vec<glib::BoxedAnyObject> =
            init.into_iter().map(glib::BoxedAnyObject::new).collect();
        self.store.extend_from_slice(&objects);
    }

    #[cfg(feature = "gnome_43")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gnome_43")))]
    /// Find the index of the first item that matches a certain function.
    pub fn find<F: FnMut(&T) -> bool>(&self, mut equal_func: F) -> Option<u32> {
        self.store.find_with_equal_func(move |obj| {
            let value = get_value::<T>(obj);
            equal_func(&value)
        })
    }

    /// Returns true if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the length of the list (without filters).
    pub fn len(&self) -> u32 {
        self.store.n_items()
    }

    /// Get the [`TypedListItem`] at the specified position.
    ///
    /// Returns [`None`] if the position is invalid.
    pub fn get(&self, position: u32) -> Option<TypedListItem<T>> {
        if let Some(obj) = self.store.item(position) {
            let wrapper = obj.downcast::<glib::BoxedAnyObject>().unwrap();
            Some(TypedListItem::new(wrapper))
        } else {
            None
        }
    }

    /// Get the visible [`TypedListItem`] at the specified position,
    /// (the item at the given position after filtering and sorting).
    ///
    /// Returns [`None`] if the position is invalid.
    pub fn get_visible(&self, position: u32) -> Option<TypedListItem<T>> {
        if let Some(obj) = self.active_model.item(position) {
            let wrapper = obj.downcast::<glib::BoxedAnyObject>().unwrap();
            Some(TypedListItem::new(wrapper))
        } else {
            None
        }
    }

    /// Insert an item at a specific position.
    pub fn insert(&mut self, position: u32, value: T) {
        self.store
            .insert(position, &glib::BoxedAnyObject::new(value));
    }

    /// Insert an item into the list and calculate its position from
    /// a sorting function.
    pub fn insert_sorted<F>(&self, value: T, mut compare_func: F) -> u32
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        let item = glib::BoxedAnyObject::new(value);

        let compare = move |first: &glib::Object, second: &glib::Object| -> Ordering {
            let first = get_value::<T>(first);
            let second = get_value::<T>(second);
            compare_func(&first, &second)
        };

        self.store.insert_sorted(&item, compare)
    }

    /// Remove an item at a specific position.
    pub fn remove(&mut self, position: u32) {
        self.store.remove(position);
    }

    /// Remove all items.
    pub fn clear(&mut self) {
        self.store.remove_all();
    }
}
