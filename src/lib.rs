#![recursion_limit="512"]
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::services::ConsoleService;
use ream::parser::Parser;

struct Model {
    link: ComponentLink<Self>,
    input: String,
    cache: String,
    output: String,
    target: Target,
    error: String,
}

impl Model {
    fn update_output(&mut self, output: String) {
        self.cache = output.to_owned();
        self.output = output;
        self.error = String::from("");
    }

    fn update(&mut self, text: String) {
        self.input = text.to_owned();
        ConsoleService::log(&text);
        let result = Parser::new(&self.input).parse_entry();
        ConsoleService::log("after parse");
        match result {
            Ok(Some(mut entry)) => {
                let output = match self.target {
                    Target::CSV => {
                        entry.to_csv_str()
                    },
                    Target::AST => {
                        entry.to_ast_str_pretty()
                    }
                };
                self.update_output(output.expect("a"));
            },
            Err(err) => {
                self.output = self.cache.to_owned();
                self.error = format!("{:?}", err);
                // ConsoleService::log(format!("{:?}", &self.output).as_str());
            },
            _ => {
                self.output = self.cache.to_owned();
                self.error = String::from("");
            },
        }
    }

}

enum Msg {
    UpdateInput(String),
    UpdateTarget(Target),
}

#[derive(Debug)]
enum Target {
    CSV,
    AST,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            input: String::from("# Title"),
            output: String::from(""),
            cache: String::from(""),
            target: Target::CSV,
            error: String::from(""),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;
        match msg {
            UpdateInput(text) => {
                self.update(text);
            },
            UpdateTarget(target) => {
                self.target = target;
                self.update(self.input.to_owned());
            },
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="container">

                <div class="header">
                    <div class="name">
                        {"REAM"}
                        <span class="version">
                        {"v0.3.1"}
                        </span>
                    </div>
                    <div class="label">
                        {"Target:"}
                    </div>
                    <div class="button-container">
                        <select
                            id="target"
                            name="target"
                        >
                            <option
                                value="CSV"
                                onclick=self.link.callback(|_| {
                                    Msg::UpdateTarget(Target::CSV)
                                })
                            >
                                {"CSV"}
                            </option>
                            <option
                                value="AST"
                                onclick=self.link.callback(|_| {
                                    Msg::UpdateTarget(Target::AST)
                                })
                            >{"AST"}</option>
                        </select>
                    </div>
                </div>

                <div class="body-container">

                    <div class="input-container">
                        <textarea
                            id="input-text"
                            name="input-text"
                            class="box"
                            oninput=self.link.callback(|text: InputData| {
                                Msg::UpdateInput(text.value)
                            })
                        >
                            {self.input.to_owned()}
                        </textarea>
                    </div>

                    <div class="output-container box">
                        <div class="output">
                        <pre>{self.output.to_owned()}</pre>
                        </div>

                        <div class="error">
                            {self.error.to_owned()}
                        </div>
                    </div>

                </div>

            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
