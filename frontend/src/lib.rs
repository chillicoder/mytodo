use futures::Future;
use seed::{fetch, Request};
use seed::{prelude::*, *};

use mytodo::{JsonApiResponse, Task};

#[derive(Clone, Debug)]
enum Direction {
    Coming,
    Going,
}

struct Model {
    tasks: Vec<Task>,
}

#[derive(Clone, Debug)]
enum Msg {
    FetchedTasks(fetch::ResponseDataResult<JsonApiResponse>),
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {}
}

fn view(model: &Model) -> impl View<Msg> {
    let greeting = match model.direction {
        Direction::Coming => "Hello, World!",
        Direction::Going => "¡Hasta la vista!",
    };
    h1![
        class! {"heading"},
        style!["height" => "100vh",
        "width" => "100vw",
        ],
        { greeting },
    ]
}

fn fetch_drills() -> impl Future<Item = Msg, Error = Msg> {
    Request::new("http://localhost:8000/tasks/").fetch_json_data(Msg::FetchedTasks)
}

fn init(_url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(fetch_drills());
    Model {
        direction: Direction::Coming,
    }
}

#[wasm_bindgen(start)]
pub fn render() {
    seed::App::build(init, update, view).finish().run();
}
