mod pages;
use pages::home::Home;
use pages::not_found::NotFound;
use pages::stories::Stories;
use yew::prelude::*;
use yew_router::prelude::*;


mod components;
mod utils;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/stories")]
    Stories,
}

pub struct App {
    navbar_active: bool,
}

pub enum Msg {
    ToggleNavbar,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App {
            navbar_active: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                {self.view_nav(&ctx)}
                <main>
                    <Switch<Route> render={Switch::render(switch)} />
                </main>
            </BrowserRouter>
        }
    }
}

impl App {
    fn view_nav(&self, ctx: &Context<Self>) -> Html {
        let Self { navbar_active } = *self;

        let active_class = if !navbar_active {
            "collapse navbar-collapse"
        } else {
            "navbar-collapse collapse show"
        };
        html! {
            <nav class="navbar navbar-expand-lg p-2 sticky-top navbar-dark bg-dark">

                <Link<Route> classes={classes!("navbar-brand")} to={Route::Home}>
                    {"Cruxial.me"}
                </Link<Route>>

                <button class="navbar-toggler" type="button" data-toggle="collapse" data-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation"
                    onclick={ctx.link().callback(|_| Msg::ToggleNavbar)}
                >
                    <span class="navbar-toggler-icon"></span>
                </button>

                <div class={classes!(active_class)} id="navbarSupportedContent">
                    <ul class="navbar-nav mr-auto">
                        <li class="nav-item active">
                                <Link<Route> classes={classes!("nav-link")} to={Route::Home}>
                                    { "Home" }
                                </Link<Route>>
                        </li>

                        <li class="nav-item">
                        <Link<Route> classes={classes!("nav-link")} to={Route::Stories}>
                        { "Stories" }
                    </Link<Route>>
                        </li>

                        <li class="nav-item">
                            <a href="https://github.com/Cruxial0" class="nav-link">
                            {"GitHub"}
                            </a>
                        </li>

                    </ul>

                </div>
            </nav>
        }
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => {
            html! { <Home /> }
        }
        Route::Stories => {
            html! { <Stories /> }
        }
        _ => {
            html! { <NotFound /> }
        }
    }
}

pub fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<App>();
}