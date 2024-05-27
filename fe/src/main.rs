use yew::{function_component, html, use_state, Callback, Html, Renderer};

use crate::problem::Problems;

#[function_component(App)]
fn app() -> Html {
    let lang = use_state(|| 999_usize);

    let make_set_lang = |l: usize| {
        let lang = lang.clone();
        Callback::from(move |_| lang.set(l))
    };

    if *lang == 999 {
        html! {
            <div class="home-buttons">
                <button onclick={make_set_lang(0)}>{ "I cannot read English" }</button>
                <button onclick={make_set_lang(1)}>{ "我不认识中文" }</button>
                <button onclick={make_set_lang(2)}>{ "अहं संस्कृतं न जानामि" }</button>
            </div>
        }
    } else {
        html! {
            <div>
                <Problems lang={*lang}/>
            </div>
        }
    }
}

fn main() {
    Renderer::<App>::new().render();
}

mod data;
mod problem;
