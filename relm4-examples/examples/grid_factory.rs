use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt};
use relm4::factory::{Factory, FactoryPrototype, FactoryVec, GridPosition};
use relm4::Sender;
use relm4::*;

struct AppWidgets {
    main: gtk::ApplicationWindow,
    gen_grid: gtk::Grid,
}

#[derive(Debug)]
enum AppMsg {
    Add,
    Remove,
    Clicked(usize),
}

struct Data {
    counter: u8,
}

struct AppModel {
    data: FactoryVec<Data>,
    counter: u8,
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = ();
    type Settings = ();
}

impl Widgets<AppModel, ()> for AppWidgets {
    type Root = gtk::ApplicationWindow;

    fn init_view(_model: &AppModel, _components: &(), sender: Sender<AppMsg>) -> Self {
        let main = gtk::ApplicationWindowBuilder::new()
            .default_width(300)
            .default_height(200)
            .build();
        let main_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .margin_end(5)
            .margin_top(5)
            .margin_start(5)
            .margin_bottom(5)
            .spacing(5)
            .build();

        let gen_grid = gtk::Grid::builder()
            .orientation(gtk::Orientation::Vertical)
            .margin_end(5)
            .margin_top(5)
            .margin_start(5)
            .margin_bottom(5)
            .row_spacing(5)
            .column_spacing(5)
            .column_homogeneous(true)
            .build();

        let add = gtk::Button::with_label("Add");
        let remove = gtk::Button::with_label("Remove");

        main_box.append(&add);
        main_box.append(&remove);
        main_box.append(&gen_grid);

        main.set_child(Some(&main_box));

        let cloned_sender = sender.clone();
        add.connect_clicked(move |_| {
            cloned_sender.send(AppMsg::Add).unwrap();
        });

        remove.connect_clicked(move |_| {
            sender.send(AppMsg::Remove).unwrap();
        });

        AppWidgets { main, gen_grid }
    }

    fn view(&mut self, model: &AppModel, sender: Sender<AppMsg>) {
        model.data.generate(&self.gen_grid, sender);
    }

    fn root_widget(&self) -> gtk::ApplicationWindow {
        self.main.clone()
    }
}

impl AppUpdate for AppModel {
    fn update(&mut self, msg: AppMsg, _components: &(), _sender: Sender<AppMsg>) -> bool {
        match msg {
            AppMsg::Add => {
                self.data.push(Data {
                    counter: self.counter,
                });
                self.counter += 1;
            }
            AppMsg::Remove => {
                self.data.pop();
            }
            AppMsg::Clicked(index) => {
                if let Some(data) = self.data.get_mut(index) {
                    data.counter = data.counter.wrapping_sub(1);
                }
            }
        }
        true
    }
}

struct FactoryWidgets {
    button: gtk::Button,
}

impl FactoryPrototype for Data {
    type Factory = FactoryVec<Self>;
    type Widgets = FactoryWidgets;
    type Root = gtk::Button;
    type View = gtk::Grid;
    type Msg = AppMsg;

    fn generate(&self, index: &usize, sender: Sender<AppMsg>) -> FactoryWidgets {
        let button = gtk::Button::with_label(&self.counter.to_string());
        let index = *index;
        button.connect_clicked(move |_| {
            sender.send(AppMsg::Clicked(index)).unwrap();
        });
        FactoryWidgets { button }
    }

    // create a chess grid pattern
    fn position(&self, index: &usize) -> GridPosition {
        let index = *index as i32;

        // add a new row for every 5th element
        let row = index / 5;
        // use every second column and move columns in uneven rows by 1
        let column = (index % 5) * 2 + row % 2;

        GridPosition {
            column,
            row,
            width: 1,
            height: 1,
        }
    }

    fn update(&self, _index: &usize, widgets: &FactoryWidgets) {
        widgets.button.set_label(&self.counter.to_string());
    }

    fn get_root(widgets: &FactoryWidgets) -> &gtk::Button {
        &widgets.button
    }
}

fn main() {
    let model = AppModel {
        data: FactoryVec::new(),
        counter: 0,
    };
    let relm = RelmApp::new(model, &());
    relm.run();
}
