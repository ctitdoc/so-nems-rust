use gloo_console as console;
use js_sys::Date;
use yew::prelude::*;
use yew::{html, Component, Context, Html};
use reqwasm::http::Request;
use serde::{Deserialize};

// Define the possible messages which can be sent to the component

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
#[derive(Clone, PartialEq, Deserialize)]
pub struct Produit {
    nom_produit: String,
}
#[derive(Clone, PartialEq, Deserialize)]
struct Commande {
    quantite_cmd: i32,
    member_id: i32,
}


pub enum Msg {
    Increment,
    Decrement,
    GetMembers,
    GetProducts,
    GetCommande,
}

pub struct App {
    value: i64, // This will store the counter value
    videos : Vec<Video>,
    products: Vec<Produit>,
    commande: Vec<Commande>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0, videos: Vec::new(), products: Vec::new(), commande:Vec::new()}


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

            Msg::GetMembers => {
                let videos = vec![
                    Video {
                        nom: "toto".to_string(),
                        prenom: "titi".to_string(),
                        date_naissance: "".to_string(),
                        adresse: "".to_string(),
                        adresse_mail: "".to_string(),
                        numero_tel: "".to_string(),
                        mot_de_passe: "".to_string(),
                        confirmation_mp: "".to_string(),
                    }
                ];
/*               let videos = use_state(|| vec![]);
                //let videos = vec![];
                {
                    let videos = videos.clone();
                    use_effect_with_deps(move |_| {
                        let videos = videos.clone();
                        wasm_bindgen_futures::spawn_local(async move {
                            let fetched_videos: Vec<Video> = Request::get(" /api/member")
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
*/
                self.videos = videos;

                true
            }
            Msg::GetProducts=>{
                let products = vec![
                    Produit{
                        nom_produit : "paté impériale".to_string(),

                }
                ];
                self.products = products;

                true
            }
            Msg::GetCommande=>{
                let commande= vec![
                    Commande{
                        quantite_cmd : 13,
                        member_id: 1,

                    }
                ];
                self.commande = commande;
                true
            }

        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let videos = self.videos.iter().map(|video| html! {
    <p>{format!("{}: {}", video.nom, video.prenom)}</p>
}).collect::<Html>();
        let products = self.products.iter().map(|produit| html! {
    <p>{format!("{}", produit.nom_produit)}</p>
}).collect::<Html>();
        let commande = self.commande.iter().map(|commande| html! {
    <p>{format!("quantité : {}, id membre: {}", commande.quantite_cmd, commande.member_id)}</p>
}).collect::<Html>();

        html! {
            <>





<header>
  <nav class="navbar-part">
    <div class="container">
      <div class="navbar-content">
        <a href="#">
          <img src="./img/nems-logo.jpg" alt=""/>
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
          /*  <a href="test-yew.html">
              <li class="fifth-link">{"test yew"}</li>
            </a>
            <a href="content_compte.html">
              <li class="fifth-link">{"contenu d'un compte"}</li>
            </a>*/
            <a href = "#" onclick={ctx.link().callback(|_| Msg::GetMembers)}> {"liste des membres"}</a>
             <a href = "#" onclick={ctx.link().callback(|_| Msg::GetCommande)}> {"Commande"}</a>
             <a href = "#" onclick={ctx.link().callback(|_| Msg::GetProducts)}> {"liste produit"}</a>



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
          <p>{"**************************************** "}</p>
          <p>{"******************************************."}</p>
          <p>{"***************************************************"}</p>
        </div>


      </div>
      <div class="Livraison">
        <div class="desc-livr">
          <h2> {"Livraison"} </h2>
          <p>{"Perimètre de livraison : Crolles"}</p>
          <p>{"Numéro livreur : 01.02.03.04.05"}</p>
        </div>
      <div class="img-livr"><img src="./img/dark-scoot.png"/></div>


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
            <div class="video">
             <h3>{"Affichage member"}</h3>

            {videos}

             <h3>{"Fin affichage member"}</h3>
         </div>

            <div class="video">
             <h3>{"Affichage produits"}</h3>

            {products}

             <h3>{"Fin affichage produits"}</h3>
         </div>
               <div class="video">
             <h3>{"Affichage commande"}</h3>

            {commande}

             <h3>{"Fin affichage commande"}</h3>
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
</>

    }
    }
}




fn main() {
yew::start_app::<App>();
}

//TODO : demain faire pour produits.