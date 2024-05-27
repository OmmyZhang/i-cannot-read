use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use yew::{function_component, html, use_state, Callback, Html};
use yew_autoprops::autoprops;

use crate::data::COLORS;

const N_OPT: usize = 3;

fn option_card(show_color: &str, text_color: &str, is_right: bool, cb: Callback<i32>) -> Html {
    html! {
        <div
            class="option-card"
            style={format!("color: {show_color}")}
            onclick={cb.reform(move |_| if is_right { 1 } else { -1 })}
        >
            { text_color }
        </div>
    }
}

#[autoprops]
#[function_component(Problem)]
fn problem(lang: usize, cb: Callback<i32>) -> Html {
    let mut rng = thread_rng();

    let selected_colors = COLORS.choose_multiple(&mut rng, N_OPT).collect::<Vec<_>>();

    let correct_answer = rng.gen_range(0..N_OPT);

    let d = rng.gen_range(0..3);

    html! {
        <div class="problem-box">
            <h2>{selected_colors[correct_answer][lang]}</h2>
            <div class="options">
                {
                    (0..N_OPT).map(|idx| {
                       option_card(
                            selected_colors[idx][0],
                            selected_colors[(idx + d) % N_OPT][lang],
                            idx == correct_answer,
                            cb.clone(),
                        )
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}

#[autoprops]
#[function_component(Problems)]
pub fn problems(lang: usize) -> Html {
    let pid = use_state(|| 1_usize);
    let score = use_state(|| 0_i32);
    let cb = {
        let score = score.clone();
        Callback::from(move |d| score.set(*score + d))
    };

    html! {
        <>
            <div class="info">{*score}</div>
            <Problem key={*pid} {lang} {cb} />
        </>
    }
}
