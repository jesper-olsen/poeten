use rand::Rng;
use urlencoding::decode;
use yew::prelude::*;
use yew::Properties;
use yew_router::prelude::*;

mod digte;

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
    #[at("/rouletten")]
    Rouletten,
    #[at("/digte")]
    Digte,
    #[at("/digt/:id")]
    Digt { id: usize },
    #[at("/tema/:id")]
    Tema { id: usize },
}

#[function_component]
fn Menu() -> Html {
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
                //<a href="/rouletten" >{"rouletten"}</a>
                <Link<Route> to={Route::Rouletten}>{"rouletten"}</Link<Route>>
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

#[function_component]
fn Rouletten() -> Html {
    let mut rng = rand::thread_rng();
    let id = rng.gen_range(0..digte::DIGTE.len());
    html! { <Digt id={id} /> }
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
            <h1> {"DIGTSSMLINGER"} </h1>
            <div style="display: grid; grid-template-columns: repeat(3, 1fr);">
            { digte::SAMLINGER.iter().map(|s| html! { <div> <Link<Route> to={Route::Samling {name: (*s).to_string()}}>{*s}</Link<Route>> </div>}).collect::<Html>() }
            </div>
        </div>
    }
}

#[function_component]
fn Tema(props: &PropsDigt) -> Html {
    let tema = props.id as u64;

    let mut l: Vec<(usize, &str)> = digte::DIGTE
        .iter()
        .enumerate()
        .filter(|(_i, (_name, temaer, _digt))| temaer & 1 << tema != 0)
        .map(|(i, (_samling, _temaer, digt))| (i, (*digt).split("\n").next().unwrap()))
        .collect();

    l.sort_by(|a, b| a.1.cmp(&b.1));

    html! {
        <div id="maincontent">
            <div>{"Digte med temaet \""}{digte::TEMAER[props.id]}{"\""}</div>
            if l.len()>0 {
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
        .map(|(i, (_samling, _temaer, digt))| (i, (*digt).split("\n").next().unwrap()))
        .collect();

    html! {
        if l.len()>0 {
            <div id="maincontent">
                    <pre>{digt} </pre>
                    if temaer & 1<<33 != 0 {
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
                    .filter(|(i,_label)| (1<<*i) & temaer != 0)
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
    let mut l: Vec<(usize, &str)> = digte::DIGTE
        .iter()
        .enumerate()
        .map(|(i, (_samling, _temaer, digt))| (i, (*digt).split("\n").next().unwrap()))
        .collect();

    l.sort_by(|a, b| a.1.cmp(&b.1));

    html! {
        <div id="maincontent">
            <div>{"Alle digte "}</div>
            if l.len()>0 {
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
fn Samling(props: &Props) -> Html {
    let name = String::from(decode(&props.name).unwrap());

    let i: usize = digte::DIGTE
        .iter()
        .enumerate()
        .filter(|(_i, (samling, _temaer, _digt))| name.eq(samling))
        .map(|(i, (_samling, _temaer, _digt))| i)
        .next()
        .unwrap();

    html! { <Digt id={i} /> }
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
Oplæsning af 347 tekster
Opdateret 14.12.2023"}</pre>
    </div>
        }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Forsiden => {
            html! { <div id="wrapper"> <Menu /> <div id="pagebody"> <Forsiden /> <Footer /> </div> </div> }
        }
        Route::Samlinger => {
            html! { <div id="wrapper"> <Menu /> <div id="pagebody"> <Samlinger /> <Footer /> </div> </div> }
        }
        Route::Samling { name } => {
            html! { <div id="wrapper"> <Menu /> <div id="pagebody"> <Samling name={name} /> <Footer /> </div> </div> }
        }
        Route::Temaer => {
            html! { <div id="wrapper"> <Menu /> <div id="pagebody"> <Temaer /> <Footer /> </div> </div> }
        }
        Route::Rouletten => {
            html! { <div id="wrapper"> <Menu /> <div id="pagebody"> <Rouletten /> <Footer /> </div> </div> }
        }
        Route::Digt { id } => {
            html! { <div id="wrapper"> <Menu /> <div id="pagebody"> <Digt id={id} /> <Footer /> </div> </div> }
        }
        Route::Digte => {
            html! { <div id="wrapper"> <Menu /> <div id="pagebody"> <Digte /> <Footer /> </div> </div> }
        }
        Route::Tema { id } => {
            html! { <div id="wrapper"> <Menu /> <div id="pagebody"> <Tema id={id} /> <Footer /> </div> </div> }
        }
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
