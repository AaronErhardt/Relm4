use super::{Components, Model, Widgets};
use crate::Sender;

impl Model for () {
    type Msg = ();
    type Widgets = ();
    type Components = ();
}

impl<ModelType, ParentModel> Widgets<ModelType, ParentModel> for ()
where
    ModelType: Model<Widgets = ()>,
    ParentModel: Model,
{
    type Root = ();

    fn init_view(
        _model: &ModelType,
        _components: &ModelType::Components,
        _sender: Sender<ModelType::Msg>,
    ) -> Self {
    }

    fn connect_parent(&self, _parent_widgets: &ParentModel::Widgets) {}

    fn root_widget(&self) -> Self::Root {}

    fn view(&mut self, _model: &ModelType, _sender: Sender<ModelType::Msg>) {}
}

impl<ParentModel: Model> Components<ParentModel> for () {
    fn init_components(_parent_model: &ParentModel, _sender: Sender<ParentModel::Msg>) {}

    fn connect_parent(&mut self, _parent_widget: &ParentModel::Widgets) {}
}
