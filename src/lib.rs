extern crate stdweb;
#[macro_use]
extern crate yew;


use stdweb::web::Date;
use yew::prelude::*;
use yew::services::ConsoleService;

pub struct Model {
    console: ConsoleService,
    value: i64,
}

pub enum Msg {
    Increment,
    Decrement,
    Bulk(Vec<Msg>),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            console: ConsoleService::new(),
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Increment => {
                self.value = self.value + 1;
                self.console.log("plus one");
            }
            Msg::Decrement => {
                self.value = self.value - 1;
                self.console.log("minus one");
            }
            Msg::Bulk(list) => for msg in list {
                self.update(msg);
                self.console.log("Bulk action");
            },
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let mut str: String = "Start".to_string();
        for _i in 0..10 {
            str = str.clone() + "<p/>";
        }

        /*
        html! {
            { for _i in 0..20 {
                <p>
                 "ok"
                </p>
            } 
            }
            <div>
                <nav class="menu",>
                    <button onclick=|_| Msg::Increment,>{ "Increment" }</button>
                    <button onclick=|_| Msg::Decrement,>{ "Decrement" }</button>
                    <button onclick=|_| Msg::Bulk(vec![Msg::Increment, Msg::Increment]),>{ "Increment Twice" }</button>
                </nav>
                <p>
                </p>
                {
                  str
                }
                <div>
                </div>
                <p>{ self.value + 1 }</p>
                <p>{ Date::new().to_string() }</p>
            </div>
        }
        */

        
        let render = |idx| html! {
            <p>
            { format!( "radi li ovo x {:2} ", idx) }
            <button>{ "OK" }</button>
            </p>
        };

        html! { // We use a fragment directly
            <div>
            <h1>
            { "Hello RUST" }
            </h1>
            </div>
            <pre>
            <div>
            { for (0..10000).map(render) }
            </div>
            </pre>
        }






    }
}

