use yew::{html, Html, virtual_dom::VNode};

pub struct Card {
    id: usize,
    title: String,
    description: String,
    image_path: String,
    tags: Vec<String>
}

/// ## Create an array of cards with an image background
/// `cards` - cards to create
pub fn create_cards(cards: Vec<Card>) -> VNode{
    let cards = cards.iter().map(|card| html! {
        <div class="col">
            <div class="card text-bg-dark" style="max-width: 14rem;">
                <img src={format!("{}", card.image_path)} class="card-img w-100" alt="ah yes"/>
                <a href={} style="color:inherit">
                    <div class="card-img-overlay" style="background-color: rgba(0, 0, 0, 0.6)">
                        <h5 class="card-title">{format!("{}", card.title)}</h5>
                        <p class="card-text">{format!("{}", card.description)}</p>
                        {format_tags(&card.tags)}
                    </div>
                </a>
            </div>
        </div>
    }).collect::<Html>();
    cards
}

pub fn format_tags(tags: &Vec<String>) -> VNode{
    let mut formatted_tags = "".to_string();
    let mut nsfw_tag = "";
    for tag in tags {
        if tag.to_uppercase() == "NSFW"{
            nsfw_tag = "nsfw, ";
            continue;
        }
        formatted_tags = formatted_tags + &format!("{t}, ", t=tag)
    }
    formatted_tags.pop();
    formatted_tags.pop();

    html!{<p class="card-text" style="text-overflow: clip; vertical-align:bottom"><small><span style="color: #ff0000">{nsfw_tag}</span>{formatted_tags}</small></p>}
}

pub fn create_sample_cards() -> Vec<Card>{
    let cards = vec![
        Card {
            id: 1,
            title: "Story 1".to_string(),
            description: "A test description to test procedural html generation (not really procedural but yeah)".to_string(),
            image_path: "/data/images/stories/1.png".to_string(),
            tags: vec!["medieval".to_string(), "action".to_string()]
        },
        Card {
            id: 2,
            title: "Story 2".to_string(),
            description: "A test description to test procedural html generation (not really procedural but yeah)".to_string(),
            image_path: "/data/images/stories/2.png".to_string(),
            tags: vec!["medieval".to_string(), "action".to_string()]
        },
        Card {
            id: 3,
            title: "Story 3".to_string(),
            description: "A test description to test procedural html generation (not really procedural but yeah)".to_string(),
            image_path: "/data/images/stories/3.png".to_string(),
            tags: vec!["medieval".to_string(), "action".to_string()]
        },
        Card {
            id: 4,
            title: "Story 4".to_string(),
            description: "A test description to test procedural html generation (not really procedural but yeah)".to_string(),
            image_path: "/data/images/stories/4.png".to_string(),
            tags: vec!["medieval".to_string(), "action".to_string()]
        },
        Card {
            id: 5,
            title: "Story 5".to_string(),
            description: "A test description to test procedural html generation (not really procedural but yeah)".to_string(),
            image_path: "/data/images/stories/5.png".to_string(),
            tags: vec!["medieval".to_string(), "action".to_string()]
        },
        Card {
            id: 6,
            title: "Story 6".to_string(),
            description: "A test description to test procedural html generation (not really procedural but yeah)".to_string(),
            image_path: "/data/images/stories/6.png".to_string(),
            tags: vec!["nsfw".to_string(), "medieval".to_string(), "action".to_string()]
        }
    ];
    cards
}