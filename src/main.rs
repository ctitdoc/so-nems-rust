use gloo_console as console;
use js_sys::Date;
use yew::prelude::*;
use yew::{html, Component, Context, Html};
use reqwasm::http::{Request, Response};
use serde::{Deserialize};
use wasm_bindgen_futures::{spawn_local};
use crate::Msg::GetProducts;

// Define the possible messages which can be sent to the component
//Test Push
#[derive(Clone, PartialEq, Deserialize)]
pub struct Video {
    nom: String,
    prenom: String,
    date_naissance: String,
    numero_tel: String,
    adresse_mail: String,
    mot_de_passe: String,
    confirmation_mp: String,
    adresse: String,

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
    GetSubscribe,
    GetSubscribeEnd,
    GetHome,
    GetAnnonce,
    GetFAQ,
    GetLaCarte,
    GetContact,
    GetCompte,
    UpdateMemberList(Vec<Video>),
    UpdateCmdList(Vec<Commande>),
    UpdateProdList(Vec<Produit>),
    Home,
}

pub struct App {
    value: i64,
    // This will store the counter value
    videos: Vec<Video>,
    products: Vec<Produit>,
    commande: Vec<Commande>,
    current_request: Msg,
}

async fn wrap<F: std::future::Future>(f: F, the_callback: yew::Callback<F::Output>) {
    console::log!("execution START of wrap fn...");
    the_callback.emit(f.await);
    console::log!("execution END of wrap fn...");
}

impl App {
    fn get_html_member_list(&self, ctx: &Context<Self>) -> Html {
        let rows = self.videos.iter().map(|video| html! {
    <tr>
            <td>{&video.nom}</td>
            <td>{&video.prenom}</td>
            <td>{&video.date_naissance}</td>
            <td>{&video.numero_tel}</td>
            <td>{&video.adresse_mail}</td>
            <td>{&video.mot_de_passe}</td>
            <td>{&video.confirmation_mp}</td>
            <td>{&video.adresse}</td>
    </tr>
        }
        ).collect::<Html>();
        /*{format!("{}: {} {} {} {} {} {} {}", video.nom, video.prenom, video.date_naissance, video.numero_tel,
                 video.adresse_mail, video.mot_de_passe, video.confirmation_mp, video.adresse)}*/
        html! {
            <section>
            <div class="member">
        <table id="admin_member">

            <tbody>
                        <div  class="main">
            <h1> {"affichage member"}</h1>
            {rows}
                                    </div>

    </tbody>
    </table>
            </div>
            </section>
    }
    }

    fn get_html_product_list(&self, ctx: &Context<Self>) -> Html {
        let rows = self.products.iter().map(|produit| html! {
           <tr>
           <td>{&produit.nom_produit}</td>
           </tr>
    //<p>{format!("{}", produit.nom_produit)}</p>
}).collect::<Html>();

        html! {
            <section>
            <div class="member">
        <table id = "admin_prod">
            <div class="main">
            <thead>
            <tr>
            <th> {"Affichage products"}</th>
            </tr>
            </thead>
            <tbody>
    {rows}
    </tbody>
    </div>
    </table>

            </div>
            </section>
    }
    }
    fn get_html_cmd_list(&self, ctx: &Context<Self>) -> Html {
        let rows = self.commande.iter().map(|commande| html! {
           <tr>
           <td><p>{"quantité commandé :"}</p>{&commande.quantite_cmd}</td>
            <td><p>{"id:"}</p>{&commande.member_id}</td>
           </tr>
    //commande.quantite_cmd, commande.member_id
}).collect::<Html>();

        html! {
            <section>
            <div class="member">
        <table id = "admin_cmd">
            <div class="main">
            <thead>
                <tr>
                    <th> {"Affichage commande"}</th>
                </tr>
            </thead>
            <tbody>
                {rows}
            </tbody>
            </div>
        </table>
            </div>
            </section>
        }
    }
    fn get_html_accueil(&self, ctx: &Context<Self>) -> Html {
        html! {
    <header>
        <div class = "nav-img">
            <div class = "img-pres" >
        </div>
        <div class = "title">
            <h1>{"Sô Nems"}</h1>
            <hr color = "black"/>
            <h2>{"spécialité maison"}</h2>
        </div>
            <div class = "construction">
            <h1>{"Site en construction"}</h1>
        </div>
        </div>
    </header>
    }
    }

    fn get_html_nav(&self, ctx: &Context<Self>) -> Html {
        html! {
            <header>
  <nav class="navbar-part">
    <div class="container">
      <div class="navbar-content">
        <a href = "#home" onclick={ctx.link().callback(|_| Msg::GetHome)}> <img src="./img/nems-logo.jpg" alt=""/> </a>
        <a href = "#home" onclick={ctx.link().callback(|_| Msg::GetHome)}> <h1> {"Sô Nems.fr"}</h1> </a>
        <div class="navbar-links">
          <ul class="navbar-link">
            <a href="#home" onclick={ctx.link().callback(|_| Msg::GetHome)}> <li class="navbar-item"> {"Acceuil"}</li></a>
            <a href="#carte" onclick={ctx.link().callback(|_| Msg::GetLaCarte)}> <li class="navbar-item">{"La Carte"}</li></a>
            <a href="#annonce" onclick={ctx.link().callback(|_| Msg::GetAnnonce)}> <li class="navbar-item">{"Annonce"}</li></a>
            <a href="#contact" onclick={ctx.link().callback(|_| Msg::GetContact)}> <li class="navbar-item">{"Contact"}</li></a>
            <a href="#compte" onclick={ctx.link().callback(|_| Msg::GetCompte)}> <li class="fifth-link">{"Mon Compte"}</li></a>
            <a href="#FAQ" onclick={ctx.link().callback(|_| Msg::GetFAQ)}> {"FAQ"} </a>
            <a href = "#admin_member" onclick={ctx.link().callback(|_| Msg::GetMembers)}> {"liste des membres"}</a>
             <a href = "#admin_cmd" onclick={ctx.link().callback(|_| Msg::GetCommande)}> {"Commande"}</a>
             <a href = "#admin_prod" onclick={ctx.link().callback(|_| Msg::GetProducts)}> {"liste produit"}</a>
            <a href = "#inscription" onclick={ctx.link().callback(|_| Msg::GetSubscribe)}> {"S'inscrire"}</a>
          </ul>
        </div>
      </div>
    </div>
  </nav>

</header>
        }
    }
    fn get_html_concept(&self, ctx: &Context<Self>) -> Html {
        html! {
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
        }
    }
    fn get_html_faq(&self, ctx: &Context<Self>) -> Html {
        html! {
            //TODO : Modification Css
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
    </section>
            }
    }
    fn get_html_footer(&self, ctx: &Context<Self>) -> Html {
        html! {
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
        }
    }


    fn get_html_inscrire(&self, ctx: &Context<Self>) -> Html {
        html! {
            <table id="inscription">
                <section>
        <div class="container">
            <div class="formulaire">
                <form id="member_subscription_form" name="member_subscription_form_name" method="post" action="#">
                    <div>
                    <p><label for="nom"> {"Nom"}</label><br/>
                        <input type="text" name="nom" id="nom" placeholder="Ex: Antoine" size="25" maxlength="100"/>
                    </p>
                    <p><label for="prenom"> {"Prénom"}</label><br/>
                        <input type="text" name="prenom" id="prenom" placeholder="Ex: Dubuisson" size="25" maxlength="100"/>
                    </p>

                    <p><label for="date-naissance"> {"Date de naissance"} </label><br/>
                        <input type="date" name="date-naissance" id="date-naissance" placeholder="Ex: 18/12/2000"
                               size="25" maxlength="100"/></p>

                    <p><label for="tel"> {"Numero de telephone"}  </label><br/>
                        <input type="tel" name="tel" id="tel" placeholder="01.02.03.04.05" size="25" maxlength="100"/></p>

                        <p><label for="adresse"> {"Adresse"}  </label><br/>
                            <input type="text" name="adresse" id="adresse" placeholder="9 rue des tuleries" size="25" maxlength="100"/></p>

                        <p><label for="ville"> {"Ville"}  </label><br/>
                            <input type="text" name="ville" id="ville" placeholder="Oulan-Bator" size="25" maxlength="100"/></p>

                        <p><label for="code-postal"> {"Code postal"} </label><br/>
                            <input type="text" name="code-postal" id="code-postal" placeholder="16066" size="25" maxlength="100"/></p>



                        <p><label for="mail"> {"Adresse mail"}  </label> <br/>
                        <input type="email" name="mail" id="mail" placeholder="Email@email.**" size="25" maxlength="100"/>
                    </p>


                    <p><label for="pass"> {"Mot de passe"} </label> <br/>
                        <input type="password" name="pass" id="pass" placeholder="*" size="25" maxlength="100"/> </p>


                    <p><label for="pass"> {"Confirmation"}  </label> <br/>
                        <input type="password" name="conf" id="conf" placeholder="*" size="25" maxlength="100"/></p>
                    </div>
                    <div>

                        <button id="TpTest" type="button" onclick={ctx.link().callback(|_| Msg::GetSubscribeEnd)}>{"Valider"}   </button>
                        <button id="test_cmd" type="button" onclick={ctx.link().callback(|_| Msg::GetProducts)}>{"Test Commande"}  </button>
                    </div>



                </form>


            </div>
        </div>
    </section>

            </table>
            }
    }
    fn get_html_inscrireFin(&self, ctx: &Context<Self>) -> Html {
        html! {
    <header>
        <div class = "nav-img">
            <div class = "img-pres" >
        </div>
        <div class = "title">
            <h1>{"Sô Nems"}</h1>
            <hr color = "black"/>
            <h2>{"Vous êtes maintenant inscrit !"}</h2>
        </div>
            <div class = "construction">
            <h1>{"Veuillez commander !"}</h1>
        </div>
        </div>
    </header>
    }
    }
    fn get_html_laCarte(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
    <header>
    <div class="banner">
        <div class="img-banner"></div>
        <div class="titre"><h1><h1> {"Notre carte"}</h1>
            <hr color="black"/>
            <h2>{"Spécialité asiatique"}</h2></h1></div>
    </div>
    </header>
    <main>
        <section>
            <div class="container">
            <div class="intro">
                <div class="paragraph">
                    <h3>{"La Carte :"}</h3>
                    <p><strong>{"Notre carte réalisée et faite maison, conviendra à toute personne aimant de près ou de loin la cuisine asiatique."} </strong></p>
                </div>

            </div>
            <div class="menu-1">
                <div class="plat-1">
                    <img src="img/nems.jpg"/>
                </div>
                <div class="desc-plat1">
                    <h3>{"Pâté impérial ..............................................0,80 €"}</h3>
                    <p>{"**********************************************"}</p>
                    <p>{"**********************************************"}</p>
                    <p>{"**********************************************"}</p>
                    <p>{"**********************************************"}</p>
                    <h3>{"Nem .........................................................0,80 €"}</h3>
                    <p>{"**********************************************"}</p>
                    <p>{"**********************************************"}</p>
                    <p>{"**********************************************"}</p>
                    <p>{"**********************************************"}</p>
                </div>
            </div>
        </div>
    </section>
    </main>
            </>
    }
    }
    fn get_html_contact(&self, ctx: &Context<Self>) -> Html {
        html! {<main>
    <section id="contact.html">
    </section>
</main>
    }
    }
    fn get_html_compte(&self, ctx: &Context<Self>) -> Html {
        html! {
            //TODO : Modification Css
            <>
            <div class="subscribe">
<header>
    <div class="banner">
        <div class="img-banner"></div>
        <div class="titre">
            <h1>  {"Mon compte"} </h1>

        </div></div>
</header>
<main>
    <section>
        <div class="container">
            <div class="formulaire">
                <form method="post" action="#">
                    <div>
                        <p><label for="identifiant"> {"Identifiant"} </label><br/>
                            <input type="text" name="identifiant" id="identifiant" placeholder="Ex: Antoine" size="25" maxlength="100"/>
                        </p>
                        <p><label for="Mp"> {"Mot de passe"} </label><br/>
                            <input type="text" name="Mp" id="Mp" placeholder="Ex: Dubuisson" size="25" maxlength="100"/>
                        </p>
                    </div>
                    <div class = "link">
                    <a href="#"> {"Mot de passe oublié"}</a>

                    <a href="inscription.html"> {"S'inscrire"}</a>
                    </div>


                </form>
            </div>
        </div>
    </section>
     </main>
            </div>
            </>
    }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0, videos: Vec::new(), products: Vec::new(), commande: Vec::new(), current_request: Msg::Home }
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
                self.current_request = Msg::GetMembers;
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
                self.current_request = Msg::GetProducts;
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
                self.current_request = Msg::GetCommande;
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

            Msg::GetSubscribe => {
                self.current_request = Msg::GetSubscribe;
                console::log!("execution of update fn / Msg::GetSubscribe...");
                true
            }

            Msg::GetSubscribeEnd => {
                self.current_request = Msg::GetSubscribeEnd;
                //TODO :
                console::log!("execution of update fn / Msg::GetSubscribeEnd");
                true
            }
            Msg::GetHome => {
                self.current_request = Msg::GetHome;
                console::log!("execution of update fn / Msg::GetHome");
                true
            }
            Msg::GetLaCarte => {
                self.current_request = Msg::GetLaCarte;
                console::log!("execution of update fn / Msg::GetLaCarte");
                true
            }
            Msg::GetAnnonce => {
                self.current_request = Msg::GetAnnonce;
                console::log!("execution of update fn / Msg::GetAnnonce");
                true
            }
            Msg::GetContact => {
                self.current_request = Msg::GetContact;
                console::log!("execution of update fn / Msg::GetContact");
                true
            }
            Msg::GetCompte => {
                self.current_request = Msg::GetCompte;
                console::log!("execution of update fn / Msg::GetCompte");
                true
            }
            Msg::GetFAQ => {
                self.current_request = Msg::GetFAQ;
                console::log!("execution of update fn / Msg::GetFAQ");
                true
            }
            Msg::GetLaCarte => {
                self.current_request = Msg::GetLaCarte;
                console::log!("execution of update fn / Msg::GetLaCarte");
                true
            }
            Msg::GetContact => {
                self.current_request = Msg::GetContact;
                console::log!("execution of update fn / Msg::GetContact");
                true
            }
            Msg::GetCompte => {
                self.current_request = Msg::GetCompte;
                console::log!("execution of update fn / Msg::GetCompte");
                true
            }
            Msg::GetAnnonce => {
                self.current_request = Msg::GetAnnonce;
                console::log!("execution of update fn / Msg::GetAnnonce");
                true
            }
            _ => {true}
        }
    }


    fn view(&self, ctx: &Context<Self>) -> Html {
        let videos = self.get_html_member_list(ctx);
        let products = self.get_html_product_list(ctx);
        let commande = self.get_html_cmd_list(ctx);
        let compte = self.get_html_compte(ctx);
        let laCarte = self.get_html_laCarte(ctx);
        let contact = self.get_html_contact(ctx);
        let accueil = self.get_html_accueil(ctx);
        let navbar = self.get_html_nav(ctx);
        let concept = self.get_html_concept(ctx);
        let faq = self.get_html_faq(ctx);
        let footer = self.get_html_footer(ctx);
        let inscrire = self.get_html_inscrire(ctx);
        let inscrireFin = self.get_html_inscrireFin(ctx);
        let main_view_content = match self.current_request {


            Msg::GetMembers =>{
                videos
            }
            Msg::GetProducts => {
                products

            }
            Msg::GetCommande =>{
                commande
            }
            Msg::GetSubscribe =>{
                inscrire
            }
            Msg::GetSubscribeEnd=>{
                inscrireFin
            }
            Msg::GetHome=>{
                accueil
            }
            Msg::GetFAQ=>{
                faq
            }
            Msg::GetAnnonce=>{
                accueil
            }
            Msg::GetCompte=>{
                compte
            }
            Msg::GetLaCarte=>{
                laCarte
            }
            Msg::GetContact=>{
                accueil
            }
            _ => {accueil}
        };

        html! {
            <>





{navbar}
<main>
{main_view_content}
</main>
{footer}
</>
/*<main>
  <section class= "Accueil" id="Acceuil">
    <div class = "admin-aff">
            {main_view_content}
    </div>
            {footer}
    </section>

</main>
</>*/

    }
    }
}


fn main() {
    yew::start_app::<App>();
}