/*use gloo_console as console;
use js_sys::Date;
use yew::{html, Component, Context, Html};
// Define the possible messages which can be sent to the component
pub enum Msg {
    Increment,
    Decrement,
}

pub struct App {
    value: i64, // This will store the counter value
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Increment => {
                self.value += 1;
                console::log!("plus one"); // Will output a string to the browser console
                true // Return true to cause the displayed change to update
            }
            Msg::Decrement => {
                self.value -= 1;
                console::log!("minus one");
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div id="all_pages_container">
            <div>
                <div class="panel">
                    // A button to send the Increment message
                    <button class="button" onclick={ctx.link().callback(|_| Msg::Increment)}>
                        { "+1" }
                    </button>

                    // A button to send the Decrement message
                    <button onclick={ctx.link().callback(|_| Msg::Decrement)}>
                        { "-1" }
                    </button>

                    // A button to send two Increment messages
                    <button onclick={ctx.link().batch_callback(|_| vec![Msg::Increment, Msg::Increment])}>
                        { "+1, +1" }
                    </button>

                </div>

                // Display the current value of the counter
                <p class="counter">
                     { self.value }
                </p>

                // Display the current date and time the page was rendered
                <p class="footer">
                    { "Rendered: " }
                    { String::from(Date::new_0().to_string()) }
                </p>
            </div>

<p> {"TODO : ajouter les pages du site comme div ici avec les images background etc..."}</p>


<header>
  <nav class="navbar-part">
    <div class="container">
      <div class="navbar-content">
        <a href="#">
          <img src="img/nems-logo.jpg" alt="Image nems"/>
        </a>
        <h1> {"Sô Nems.fr"}</h1>

        <div class="navbar-links">
          <ul class="navbar-link">
            <a href="index.html">
              <li class="navbar-item"> {"Acceuil"}</li>
            </a>
            <a href="La-carte.html">
              <li class="navbar-item">{"La Carte"}</li>
            </a>
            <a href="Annonce">
              <li class="navbar-item">{"Annonce"}</li>
            </a>
            <a href="#contact">
              <li class="navbar-item">{"Contact"}</li>
            </a>
            <a href="mon-compte.html">
              <li class="fifth-link">{"Mon Compte"}</li>
            </a>
            <a href="test-yew.html">
              <li class="fifth-link">{"test yew"}</li>
            </a>
            <a href="content_compte.html">
              <li class="fifth-link">{"contenu d'un compte"}</li>
            </a>



          </ul>

        </div>

      </div>
    </div>
  </nav>

</header>
            <header>
  <div class="nav-img">
    <div class={"img-pres"}>
    </div>
    <div class="title">
      <h1>{"Sô Nems"}</h1>
      <hr color="black"/>
      <h2>{"spécialité maison"}</h2>
    </div>
  </div>
</header>


<main>
  <section id="Acceuil">
    <div class="Colonne">
      <div class="Concept">
        <div class="desc-conc">
          <h2>{"Sô Nems ?"}</h2>
          <p>{"****************************************"}</p>
          <p>{"******************************************"}</p>
          <p>{"***************************************************"}</p>
        </div>


      </div>
    <div class="Livraison">
        <div class="desc-livr">
          <h2> {"Livraison "}</h2>
          <p>{"Perimètre de livraison : Crolles"}</p>
          <p>{"Numéro livreur : 01.02.03.04.05"}</p>
        </div>
      <div class="img-livr"><img src="img/dark-scoot.png" alt=""/></div>


      </div>
    </div>
    <section>
      <div class="FAQ">
        <div class="FAQ-content">
          <h2> {"F.A.Q"}</h2>
          <p>{"Les ingrédients achetés à l'épicerie chinoise à Grenoble."}</p>
          <p>{"Les livraisons ne sont pas toujours proposées."}</p>
          <p>{"La maison correspond à la dernière maison de l'allée en gravier."}</p>
          <p>{"Les nems sont princialement fait de porc mais la chef peut en faire d'autre si il y eu
            demande au préalable."}</p>
        </div>
      </div>
    </section>
  </section>
<footer id= "contact" class= "footer">
    <table class="footer-table">
      <tbody>
      <tr class ="tr1">
        <td class = "icon">
          <i class="fas fa-phone-alt"></i>
        </td>
        <td class="info">
          <p>{"01.02.03.04.05"}</p>
        </td>
      </tr>
      <tr class ="tr2">
        <td class = "icon">
          <i class="fas fa-map-marker-alt"></i>
        </td>
        <td class="info">
          <p>{"18 rue Trump"}</p>
        </td>
      </tr>
      <tr class ="tr3">
      <td class = "icon">
        <i class="fas fa-clock"></i>
      </td>
      <td class="info">
        <p> {"Livraison et à Emporter"}</p>
      </td>
      </tr>
      </tbody>
    </table>
  </footer>
</main>

</div>


    }
    }
}

fn main() {
    yew::start_app::<App>();
}*/

use yew::prelude::*;

use reqwasm::http::Request;
use serde::{Deserialize};

#[derive(Clone, PartialEq, Deserialize)]
pub struct Video {
    nom: String,
    prenom: String,
    date_naissance: String,
    numero_tel: String,
    adresse_mail:String,
    mot_de_passe: String,
    confirmation_mp: String,
    adresse : String,

}
#[derive(Properties, PartialEq)]
pub struct VideosListProps {
    videos: Vec<Video>,
    on_click: Callback<Video>
}

#[derive(Clone, Properties, PartialEq)]
pub struct VideosDetailsProps {
    video: Video,
}

#[function_component(VideoDetails)]
pub fn video_details(VideosDetailsProps { video }: &VideosDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ video.nom.clone() }</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    }
}

#[function_component(VideosList)]
pub fn videos_list(VideosListProps { videos, on_click }: &VideosListProps) -> Html {
    let on_click = on_click.clone();
    videos.iter().map(|video| {
        let on_video_select = {
            let on_click = on_click.clone();
            let video = video.clone();
            Callback::from(move |_| {
                on_click.emit(video.clone())
            })
        };

        html! {

            <p onclick={on_video_select}>{format!("{}: {}", video.nom, video.prenom)}</p>
        }
    }).collect()
}
#[function_component(App)]
pub fn app() -> Html {
    let selected_video = use_state(|| None);

    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| {
            selected_video.set(Some(video))
        })
    };

    let details = selected_video.as_ref().map(|video| html! {
        <VideoDetails video={video.clone()} />
    });


    let videos = use_state(|| vec![]);
        {
                let videos = videos.clone();
                use_effect_with_deps(move |_| {
                        let videos = videos.clone();
                        wasm_bindgen_futures::spawn_local(async move {
                            let fetched_videos: Vec<Video> = Request::get("/api/member")
                                        .send()
                                        .await
                                        .unwrap()
                                        .json()
                                        .await
                                        .unwrap();
                                videos.set(fetched_videos);
                            });
                        || ()
                        }, ());
            }

       html! {
    <>

           <header>
            <nav class="navbar-part">
            <div class="container">
      <div class="navbar-content">
          <img src="/img/nems-logo.jpg" alt="Image nems"/>
        <h1> {"Sô Nems.fr"}</h1>

        <div class="navbar-links">
          <ul class="navbar-link">
              <li class="navbar-item"> {"Acceuil"}</li>
              <li class="navbar-item">{"La Carte"}</li>
              <li class="navbar-item">{"Annonce"}</li>
              <li class="navbar-item">{"Contact"}</li>
              <li class="fifth-link">{"Mon Compte"}</li>
              <li class="fifth-link">{"test yew"}</li>
              <li class="fifth-link">{"contenu d'un compte"}</li>
          </ul>
        </div>
      </div>
    </div>
  </nav>

</header>
<header>
<div class="nav-img">
  <div class="img-pres">
  </div>
  <div class="title">
    <h1>{"Sô Nems"}</h1>
    <h2>{"spécialité maison"}</h2>
  </div>
</div>
</header>


<main>
<section id="Acceuil">
  <div class="Colonne">
    <div class="Concept">
      <div class="desc-conc">
        <h2>{"Sô Nems ?"}</h2>
        <p>{"****************************************"} </p>
        <p>{"******************************************."}</p>
        <p>{"***************************************************"}</p>
      </div>


    </div>
    <div class="Livraison">
      <div class="desc-livr">
        <h2> {"Livraison "}</h2>
        <p>{"Perimètre de livraison : Crolles"}</p>
        <p>{"Numéro livreur : 01.02.03.04.05"}</p>
      </div>
    <div class="img-livr"><img src="/img/dark-scoot.png"/></div>


    </div>
  </div>
  <section>
    <div class="FAQ">
      <div class="FAQ-content">
        <h2> {"F.A.Q"}</h2>
        <p>{"Les ingrédients achetés à l'épicerie chinoise à Grenoble."}</p>
        <p>{"Les livraisons ne sont pas toujours proposées."}</p>
        <p>{"La maison correspond à la dernière maison de l'allée en gravier."}</p>
        <p>{"Les nems sont princialement fait de porc mais la chef peut en faire d'autre si il y eu
          demande au préalable."}</p>
      </div>
    </div>
  </section>
</section>
<footer id= "contact" class= "footer">
  <table class="footer-table">
    <tbody>
    <tr class ="tr1">
      <td class = "icon">
        <i class="fas fa-phone-alt"></i>
      </td>
      <td class="info">
        <p>{"01.02.03.04.05"}</p>
      </td>
    </tr>
    <tr class ="tr2">
      <td class = "icon">
        <i class="fas fa-map-marker-alt"></i>
      </td>
      <td class="info">
        <p>{"18 rue Trump"}</p>
      </td>
    </tr>
    <tr class ="tr3">
    <td class = "icon">
      <i class="fas fa-clock"></i>
    </td>
    <td class="info">
      <p> {"Livraison et à Emporter"}</p>
    </td>
    </tr>
    </tbody>
  </table>
</footer>
</main>
        //    <main>
        // <div class="video">
        //     <h3>{"Videos to watch"}</h3>
        //     <VideosList videos={(*videos).clone()} on_click={on_video_select.clone()} />
        //     <h4>{"Fin affichage member"}</h4>
        // </div>
        //
        //     { for details }
        //    </main>
           </>
}
}



fn main() {
    yew::start_app::<App>();
}