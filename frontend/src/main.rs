use rand::RngExt;
use urlencoding::decode;
use yew::Properties;
use yew::prelude::*;
use yew_router::prelude::*;

mod digte;

const TEMA_OPLÆSNING: u64 = 1u64 << 33;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
}

#[derive(Properties, PartialEq)]
pub struct PropsDigt {
    pub id: usize,
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Forsiden,
    #[at("/samlinger")]
    Samlinger,
    #[at("/samling/:name")]
    Samling { name: String },
    #[at("/temaer")]
    Temaer,
    #[at("/rouletten/:id")]
    Rouletten { id: usize },
    #[at("/digte")]
    Digte,
    #[at("/digt/:id")]
    Digt { id: usize },
    #[at("/tema/:id")]
    Tema { id: usize },
}

#[function_component]
fn Menu() -> Html {
    let navigator = use_navigator().unwrap();
    let go_rouletten = Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        let mut rng = rand::rng();
        let id = rng.random_range(0..digte::DIGTE.len());
        navigator.push(&Route::Rouletten { id });
    });

    html! {
          <>
          <div id="header" >
            <img src="/Images/havban5.jpg"  alt="Den Gamle Poet"  border="0"  width="656"  height="100" />
          </div>
          <div id="menu" >
            <ul>
              <li>
                <Link<Route> to={Route::Forsiden}>{"forsiden"}</Link<Route>>
              </li>
              <li>
                <Link<Route> to={Route::Samlinger}>{"digtsamlinger"}</Link<Route>>
              </li>
              <li>
                <Link<Route> to={Route::Temaer}>{"temaer"}</Link<Route>>
              </li>
              <li>
                  <a href="/rouletten" onclick={go_rouletten}>{"🎲 rouletten"}</a>
              </li>
            </ul>
          </div>
          </>
    }
}

#[function_component]
fn Footer() -> Html {
    html! {
      <div id="footer" >
        <p style="font-size: small" >
          {"Alle tekster \u{00a9} "}
          <a href="mailto:dengamlepoet&#64;gmail.com" >{"F. \u{00d8}stergaard"}</a>
        </p>
      </div>
    }
}

//#[function_component]
//fn Rouletten() -> Html {
//    let id = use_state(|| {
//        let mut rng = rand::rng();
//        rng.random_range(0..digte::DIGTE.len())
//    });
//    html! { <Digt id={*id} /> }
//}

#[function_component]
fn Rouletten() -> Html {
    let id = use_state(|| {
        let mut rng = rand::rng();
        rng.random_range(0..digte::DIGTE.len())
    });

    let reroll = {
        let id = id.clone();
        Callback::from(move |_| {
            let mut rng = rand::rng();
            id.set(rng.random_range(0..digte::DIGTE.len()));
        })
    };

    html! {
        <>
            <Digt id={*id} />
            <div id="sidebar">
                <button onclick={reroll}>{"🎲 Nyt digt"}</button>
            </div>
        </>
    }
}

#[function_component]
fn Temaer() -> Html {
    html! {
        <div id="maincontent">
            <h1> {"TEMAER"} </h1>
            <div style="display: grid; grid-template-columns: repeat(3, 1fr);">
            { digte::TEMAER.iter().enumerate().map(|(i,s)| html! { <div> <Link<Route> to={Route::Tema {id: i}}>{*s}</Link<Route>> </div>}).collect::<Html>() }
            //{ digte::TEMAER.iter().map(|s| html! { <div> {*s}</div>}).collect::<Html>() }
            </div>
        </div>
    }
}

#[function_component]
fn Samlinger() -> Html {
    html! {
        <div id="maincontent">
            <h1> {"DIGTSAMLINGER"} </h1>
            <div style="display: grid; grid-template-columns: repeat(3, 1fr);">
            { digte::SAMLINGER.iter().map(|s| html! { <div> <Link<Route> to={Route::Samling {name: (*s).to_string()}}>{*s}</Link<Route>> </div>}).collect::<Html>() }
            </div>
        </div>
    }
}

fn first_line(s: &str) -> &str {
    s.lines().next().unwrap_or("")
}

#[function_component]
fn Tema(props: &PropsDigt) -> Html {
    let tema = props.id as u64;

    let mut l: Vec<(usize, &str)> = digte::DIGTE
        .iter()
        .enumerate()
        .filter(|(_i, (_name, temaer, _digt))| temaer & (1u64 << tema) != 0)
        .map(|(i, (_samling, _temaer, digt))| (i, first_line(*digt)))
        .collect();

    l.sort_by(|a, b| a.1.cmp(b.1));

    html! {
        <div id="maincontent">
            <div>{"Digte med temaet \""}{digte::TEMAER[props.id]}{"\""}</div>
            if !l.is_empty() {
                <ol>
                {l.iter()
                  .map(|(i,s)| html! {<li><Link<Route> to={Route::Digt {id: *i}}>{*s}</Link<Route>></li>})
                  .collect::<Html>()}
                </ol>
            }
        </div>
    }
}

#[function_component]
fn Digt(props: &PropsDigt) -> Html {
    let (samling, temaer, digt) = digte::DIGTE[props.id];

    let l: Vec<(usize, &str)> = digte::DIGTE
        .iter()
        .enumerate()
        .filter(|(_i, (name, _temaer, _digt))| (*name).eq(samling))
        .map(|(i, (_samling, _temaer, digt))| (i, first_line(*digt)))
        .collect();

    html! {
        if !l.is_empty() {
            <div id="maincontent">
                    <pre>{digt} </pre>
                    if temaer & TEMA_OPLÆSNING != 0 {
                          <br/> <br/> <br/>
                          <audio controls=true>
                              <source src={format!("https://storage.googleapis.com/poeten-281913-wav/{}.mp3",props.id)} type="audio/mpeg" />
                              //<source src={format!("/MP3/{}.mp3",props.id)} type="audio/mpeg" />
                              {"Your browser does not support the audio element."}
                          </audio>
                    }
                    <br/> <br/> <br/>
            </div>
            <div id="sidebar"> <div id="subnav">
                <br/>
                <dl>
                <dt style="font-size:small"> {"Digtets temaer:"} </dt>
                <ul>
                {digte::TEMAER
                    .iter()
                    .enumerate()
                    .filter(|(i,_label)| (1u64 << *i) & temaer != 0)
                    .map(|(i,label)| html! {<li> <Link<Route> to={Route::Tema {id: i}}>{*label}</Link<Route>> </li>})
                    .collect::<Html>()}
                </ul>
                <br/>
                <dt style="font-size:small">{"Fra samlingen \""}{samling}{"\":"}</dt>
                <dd> <ul>
                {l.iter()
                  .map(|(i,s)| html! {<li><Link<Route> to={Route::Digt {id: *i}}>{*s}</Link<Route>></li>})
                  .collect::<Html>()}
                </ul></dd></dl>
                </div>
            </div>
        }
    }
}

#[function_component]
fn Digte() -> Html {
    // Only re-calculate/sort if the underlying data or dependencies change
    let list = use_memo((), |_| {
        let mut l: Vec<(usize, &str)> = digte::DIGTE
            .iter()
            .enumerate()
            .map(|(i, (_samling, _temaer, digt))| (i, first_line(*digt)))
            .collect();
        l.sort_by(|a, b| a.1.cmp(b.1));
        l
    });

    html! {
        <div id="maincontent">
            <div>{"Alle digte "}</div>
            <ol>
                {list.iter().map(|(i, s)| html! {
                    <li><Link<Route> to={Route::Digt {id: *i}}>{*s}</Link<Route>></li>
                }).collect::<Html>()}
            </ol>
        </div>
    }
}

#[function_component]
fn Samling(props: &Props) -> Html {
    let decoded_result = decode(&props.name).map(|s| s.into_owned());
    let poem_index = decoded_result.ok().and_then(|name| {
        digte::DIGTE
            .iter()
            .position(|(samling, _, _)| name.eq(*samling))
    });
    match poem_index {
        Some(i) => html! { <Digt id={i} /> },
        None => html! {
            <div id="maincontent">
                <h1>{"Samling ikke fundet"}</h1>
                <p>{"Beklager, vi kunne ikke finde den ønskede digtsamling."}</p>
                <Link<Route> to={Route::Samlinger}>{"Tilbage til oversigten"}</Link<Route>>
            </div>
        },
    }
}

#[function_component]
fn Forsiden() -> Html {
    html! {
        <div id="maincontent" >
    <pre>{"På denne side har du mulighed for at læse digte af en gammel
poet, der sidder alene ved sin computer et sted i Danmark og
meddeler sine tanker til verden.






Som fodspor i
det våde sand ved
havet
slægters stille
stræb







1726 tekster fra 1962 til 2023
Oplæsning af 346 tekster
Opdateret 14.12.2023"}</pre>
    </div>
        }
}

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
fn Layout(props: &LayoutProps) -> Html {
    html! {
        <div id="wrapper">
            <Menu />
            <div id="pagebody">
                { for props.children.iter() }
                <Footer />
            </div>
        </div>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Forsiden => html! { <Forsiden /> },
        Route::Samlinger => html! { <Samlinger /> },
        Route::Samling { name } => html! { <Samling name={name} /> },
        Route::Temaer => html! { <Temaer /> },
        Route::Rouletten { id } => html! { <Digt id={id} /> },
        Route::Digt { id } => html! { <Digt id={id} /> },
        Route::Digte => html! { <Digte /> },
        Route::Tema { id } => html! { <Tema id={id} /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Layout>
                <Switch<Route> render={switch} />
            </Layout>
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
