use gloo_console as console;
use js_sys::Date;
use yew::prelude::*;
use yew::{html, Component, Context, Html};
use reqwasm::http::{Request, Response};
use serde::{Deserialize};
use wasm_bindgen_futures::{spawn_local};

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
pub struct Commande {
    quantite_cmd: i32,
    member_id: i32,
}


pub enum Msg {
    Increment,
    Decrement,
    GetMembers,
    GetProducts,
    GetCommande,
    UpdateMemberList(Vec<Video>),
    UpdateCmdList(Vec<Commande>),
    UpdateProdList(Vec<Produit>)

}

pub struct App {
    value: i64, // This will store the counter value
    videos : Vec<Video>,
    products: Vec<Produit>,
    commande: Vec<Commande>,
}

async fn wrap<F: std::future::Future>(f: F, the_callback: yew::Callback<F::Output>) {
        console::log!("execution START of wrap fn...");
        the_callback.emit(f.await);
        console::log!("execution END of wrap fn...");
    }

impl App {
    fn get_html_member_list(&self, ctx: &Context<Self>) -> Html {
        self.videos.iter().map(|video| html! {
    <p>{format!("{}: {} {} {} {} {} {} {}", video.nom, video.prenom, video.date_naissance, video.numero_tel,
                video.adresse_mail, video.mot_de_passe, video.confirmation_mp, video.adresse)}</p>
}).collect::<Html>()
    }

    fn get_html_product_list(&self, ctx: &Context<Self>) -> Html {
        self.products.iter().map(|produit| html! {
    <p>{format!("{}", produit.nom_produit)}</p>
}).collect::<Html>()
    }
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

            Msg::UpdateMemberList(vids) => {
                self.videos = vids;
                true
            }

            Msg::GetMembers => {
                console::log!("execution START of update fn / Msg::GetMembers...");
                spawn_local(
                    wrap(
                        async {
                            console::log!("execution START of Request::get(\"/api/member\")...");
                            let fetched_videos = Request::get("/api/member")
                                .send()
                                .await
                                .unwrap()
                                .json()
                                .await
                                .unwrap();
                            console::log!("execution END of Request::get(\"/api/member\")...");
                            fetched_videos
                        },
                        _ctx.link().callback(|fetched_videos| Msg::UpdateMemberList(fetched_videos)))
                );

                console::log!("execution END of update fn / Msg::GetMembers ");
                true
            }

            Msg::UpdateProdList(vids) => {
                self.products = vids;
                true
            }
            Msg::GetProducts => {
                console::log!("execution START of update fn / Msg::GetProducts...");
                spawn_local(
                    wrap(
                        async {
                            console::log!("execution START of Request::get(\"/api/produit\")...");
                            let fetched_videos = Request::get("/api/produit")
                                .send()
                                .await
                                .unwrap()
                                .json()
                                .await
                                .unwrap();
                            console::log!("execution END of Request::get(\"/api/produit\")...");
                            fetched_videos
                        },
                        _ctx.link().callback(|fetched_videos| Msg::UpdateProdList(fetched_videos)))
                );

                console::log!("execution END of update fn / Msg::GetPoducts ");
                true
            }

            Msg::UpdateCmdList(vids) => {
                self.commande = vids;
                true
            }
            Msg::GetCommande => {
                console::log!("execution START of update fn / Msg::GetCommande...");
                spawn_local(
                    wrap(
                        async {
                            console::log!("execution START of Request::get(\"/api/commande\")...");
                            let fetched_videos = Request::get("/api/commande")
                                .send()
                                .await
                                .unwrap()
                                .json()
                                .await
                                .unwrap();
                            console::log!("execution END of Request::get(\"/api/commande\")...");
                            fetched_videos
                        },
                        _ctx.link().callback(|fetched_videos| Msg::UpdateCmdList(fetched_videos)))
                );

                console::log!("execution END of update fn / Msg::GetCommande ");
                true
            }
        }
    }




    fn view(&self, ctx: &Context<Self>) -> Html {
        let videos = self.get_html_member_list(ctx);
        let products = self.get_html_product_list(ctx);
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
            <a href = "#FAQbis"> {"FAQ"} </a>
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
    <section id = "FAQbis">
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
            <table>
                <thead>
                    <tr>
                        <th> {"Affichage member"}</th>
                    </tr>

                </thead>
                <tbody>
                    <tr> {videos} </tr>
                </tbody>
            </table>
            /*<div class="video">
             <h3>{"Affichage member"}</h3>

            {videos}

             <h3>{"Fin affichage member"}</h3>
         </div>*/

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
