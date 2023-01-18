
use yew::*;

pub struct Story{
    id: u8,
}

impl Component for Story{
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Story { id: use_context::<u8>().expect("no."), }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html{

        html!{
            <div classes="">
                <link href="/data/js/bootstrap-5.3.0-dist/css/bootstrap.min.css" rel="stylesheet"/>
                <title>{"Story"}</title>
                <script src="/data/js/bootstrap-5.3.0-dist/js/bootstrap.bundle.min.js"></script>
                <div class="d-flex justify-content-center m-50">
                    <h1 class="text-truncate text-primary">
                        <div class="text-primary">{"NovelAI Stories"}</div>
                    </h1>
                </div>
                <div>{self.id}</div>
            </div>
        }

    }
}