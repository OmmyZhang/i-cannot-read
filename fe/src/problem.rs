use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use yew::{function_component, html, use_memo, use_state, Callback, Html};
use yew_autoprops::autoprops;
use yew_hooks::use_interval;

use crate::data::{COLORS, N_LANG};

const N_OPT: usize = 3;
const T: u8 = 150;
const N_PROBLEM: usize = 100;

fn option_card(show_color: &str, text_color: &str, is_right: bool, cb: Callback<i32>) -> Html {
    html! {
        <div
            class="option-card"
            style={format!("color: {show_color}")}
            onclick={cb.reform(move |_| if is_right { 1 } else { -2 })}
        >
            { text_color }
        </div>
    }
}

struct ProblemData {
    colors: Vec<&'static [&'static str; N_LANG]>,
    correct_answer: usize,
    d: usize,
}

#[autoprops]
#[function_component(Problem)]
fn problem(lang: usize, cb: Callback<i32>) -> Html {
    let pbm = use_memo((), move |_| {
        let mut rng = thread_rng();
        let colors = COLORS.choose_multiple(&mut rng, N_OPT).collect::<Vec<_>>();

        ProblemData {
            colors,
            correct_answer: rng.gen_range(0..N_OPT),
            d: rng.gen_range(0..3),
        }
    });

    let rest_time = use_state(|| T);

    {
        let rest_time = rest_time.clone();
        let cb = cb.clone();
        use_interval(
            move || {
                if *rest_time == 0 {
                    cb.emit(-1);
                } else {
                    rest_time.set(*rest_time - 1);
                }
            },
            10,
        );
    }

    let ProblemData {
        colors,
        correct_answer,
        d,
    } = &*pbm;

    html! {
        <div class="problem-box">
            <div class="time-prosess">
                <progress
                    max={T.to_string()}
                    value={(T - *rest_time).to_string()}
                />
            </div>
            <h2>{colors[*correct_answer].join(", ")}</h2>
            <div class="options">
                {
                    (0..N_OPT).map(|idx| {
                       option_card(
                            colors[idx][0],
                            colors[(idx + d) % N_OPT][lang],
                            idx == *correct_answer,
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
        let pid = pid.clone();
        Callback::from(move |d| {
            score.set(*score + d);
            pid.set(*pid + 1);
        })
    };

    if *pid <= N_PROBLEM {
        html! {
            <>
                <div class="info">
                    <div>{ format!("{} / {}", *pid, N_PROBLEM) }</div>
                    <div>{ *score }</div>
                </div>
                <Problem key={*pid} {lang} {cb} />
            </>
        }
    } else {
        html! {
            <h1>{ *score }</h1>
        }
    }
}
