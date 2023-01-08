use yew::prelude::*;

use crate::utils::createcards;

pub struct Stories;

impl Component for Stories {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Stories {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {

        let cards = createcards::create_cards(createcards::create_sample_cards());

        html! {
            <div classes="">
                <link href="/data/js/bootstrap-5.3.0-dist/css/bootstrap.min.css" rel="stylesheet"/>
                <title>{"Stories"}</title>
                <script src="/data/js/bootstrap-5.3.0-dist/js/bootstrap.bundle.min.js"></script>

                <div class="d-flex justify-content-center m-50">
                    <h1 class="text-truncate text-primary">
                        <div class="text-primary">{"NovelAI Stories"}</div>
                    </h1>
                </div>
                <div class="d-flex justify-content-center mx-5">
                    <h3 class="text-truncate text-secondary">
                        <div class="text-secondary">{"A collection of all my NovelAI stories that I found worth keeping."}</div>
                    </h3>
                </div>
                <div class="container">
                    <div class="row row-cols-5 g-3 my-5">
                        { cards }
                    </div>
                </div>
            </div>
        }
    }
}