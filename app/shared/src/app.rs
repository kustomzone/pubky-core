use crux_core::render::Render;
use serde::{Deserialize, Serialize};

// ANCHOR: model
#[derive(Default, Serialize)]
pub struct Model {
    count: u64,
}

// ANCHOR_END: model

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ViewModel {
    pub screen: Screen,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Screen {
    Red,   
    Black,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Event {
    Noop,
}

#[cfg_attr(feature = "typegen", derive(crux_core::macros::Export))]
#[derive(crux_core::macros::Effect)]
pub struct Capabilities {
    pub render: Render<Event>,
}

#[derive(Default)]
pub struct App;

impl crux_core::App for App {
    type Model = Model;
    type Event = Event;
    type ViewModel = ViewModel;
    type Capabilities = Capabilities;

    fn update(&self, _msg: Self::Event, _model: &mut Self::Model, _caps: &Self::Capabilities) {}

    fn view(&self, _model: &Self::Model) -> Self::ViewModel {
        Self::ViewModel {
            screen: Screen::Red,
        }
    }
}
