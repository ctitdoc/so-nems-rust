use gloo_console as console;
use js_sys::Date;
use yew::prelude::*;
use yew::{html, Component, Context, Html, TargetCast};
use reqwasm::http::{Request, Response};
use serde::{Serialize,Deserialize};
use wasm_bindgen_futures::{spawn_local};
use crate::Msg::GetProducts;
use web_sys::HtmlInputElement;
use web_sys::Storage;
use web_sys::Window;
use std::fmt;


// Define the possible messages which can be sent to the component
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Member {
    nom: String,
    prenom: String,
    date_naissance: String,
    numero_tel: String,
    adresse_mail: String,
    mot_de_passe: String,
    confirmation_mp: String,
    adresse: String,
}

#[derive(Clone, PartialEq, Serialize,Deserialize)]
pub struct Produit {
    nom_produit: String,
    ingredients: String,
    prix: f64,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct Commande {
    quantite_cmd: i32,
    member_id: i32,
}

//TODO : Création struct Login
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Login {
    adresse_mail: String,
    mot_de_passe: String,
}

pub enum Msg {
    Increment,
    Decrement,
    GetMembers,
    GetProducts,
    GetCommande,
    GetMember,
    GetMemberEnd,
    GetHome,
    GetAnnonce,
    GetFAQ,
    GetProductFrom,
    GetLaCarte,
    GetContact,
    GetCompte,
    UpdateMemberList(Vec<Member>),
    UpdateCmdList(Vec<Commande>),
    UpdateProdList(Vec<Produit>),
    Home,
    ProductName(String),
    IngredientName(String),
    PriceNumber(f64),
    GetRecordProduct,
    GetRecordProductStatus(String),
    MemberName(String),
    MemberSurname(String),
    MemberPassword(String),
    MemberVerifyPassword(String),
    MemberAdress(String),
    MemberMailAdress(String),
    MemberPhoneNumber(String),
    MemberBirthday(String),
    GetRecordMember,
    GetRecordMemberStatus(String),
    GetLoginMember,
    UpdateLoginMember(Vec<Member>),
}

pub struct App {
    value: i64,
    // This will store the counter value
    members: Vec<Member>,
    products: Vec<Produit>,
    commande: Vec<Commande>,
    current_request: Msg,
    product: Option<Produit>,
    member: Option<Member>,
}

async fn wrap<F: std::future::Future>(f: F, the_callback: yew::Callback<F::Output>) {
    console::log!("execution START of wrap fn...");
    the_callback.emit(f.await);
    console::log!("execution END of wrap fn...");
}

pub fn get_storage() -> Option<web_sys::Storage> {
    let window = web_sys::window().unwrap();

    match window.session_storage() {
        Ok(Some(session_storage)) => {
            Some(session_storage)
        },
        Err(_) => None,
        Ok(None) => None
    }
}
pub fn get_item(name : &str) -> String {
    get_storage().unwrap().get_item(name).unwrap().unwrap()
}
pub fn set_item(name : &str, value: &str){
    get_storage().unwrap().set_item(name,value);
}

impl App {
    fn get_html_member_list(&self, ctx: &Context<Self>) -> Html {
        let rows = self.members.iter().map(|members| html! {
        <tr>
            <td>{&members.nom}</td>
            <td>{&members.prenom}</td>
            <td>{&members.date_naissance}</td>
            <td>{&members.numero_tel}</td>
            <td>{&members.adresse_mail}</td>
            <td>{&members.mot_de_passe}</td>
            <td>{&members.confirmation_mp}</td>
            <td>{&members.adresse}</td>
        </tr>
        }
        ).collect::<Html>();
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
                <td>{&produit.ingredients}</td>
                <td>{&produit.prix}</td>
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
            <a href = "#inscription" onclick={ctx.link().callback(|_| Msg::GetMember)}> {"S'inscrire"}</a>
            <a href = "#product" onclick={ctx.link().callback(|_| Msg::GetProductFrom)}> {"nouveau produit"}</a>
          </ul>
        </div>
      </div>
    </div>
  </nav>

</header>
        }
    }

    fn get_html_faq(&self, ctx: &Context<Self>) -> Html {
        html! {
            //TODO : Modification Css
            <section id = "FAQbis">
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

    fn get_html_product_form(&self, ctx: &Context<Self>) -> Html {
        let on_input_change = ctx.link().callback(|e: Event| {
            Msg::ProductName(e.target_unchecked_into::<HtmlInputElement>().value())
        });
        let on_input_change_ingredient = ctx.link().callback(|e: Event| {
            Msg::IngredientName(e.target_unchecked_into::<HtmlInputElement>().value())
        });
        let on_input_change_price = ctx.link().callback(|e: Event| {
            let str = e.target_unchecked_into::<HtmlInputElement>().value();
            let prix : f64 = str.parse::<f64>().unwrap();
            Msg::PriceNumber(prix)
        });

        html! {
        <table id="product">
            <section>
            <div class="container">
            <div class="formulaire">
            <form id="product_form" name="product_form" method="post" action="#">
            <div>
            <p><label for="product"> {"produit : "}  </label><br/>
            <input onchange ={on_input_change} type="text" name="product" id="product" placeholder="nems" size="25" maxlength="100"/></p>
            <p><label for="ingredients"> {"ingredient : "}  </label><br/>
            <input onchange ={on_input_change_ingredient} type="text" name="ingredients" id="ingredients" placeholder="sucre" size="25" maxlength="100"/></p>
            <p><label for="price"> {"prix : "}  </label><br/>
            <input onchange ={on_input_change_price} type="number" name="price" id="price" placeholder="sucre" size="25" maxlength="100"/></p>
            </div>
            <div>
            <button id="#" type="button" onclick={ ctx.link().callback(|_| Msg::GetRecordProduct)}>{"create"}  </button>
            </div>
            </form>


            </div>
            </div>
            </section>

            </table>
            }
    }

    fn get_html_inscrire(&self, ctx: &Context<Self>) -> Html {
        let on_input_change_member_name = ctx.link().callback(|e: Event| {
            Msg::MemberName(e.target_unchecked_into::<HtmlInputElement>().value())
        });
        let on_input_change_member_surname = ctx.link().callback(|e: Event| {
            Msg::MemberSurname(e.target_unchecked_into::<HtmlInputElement>().value())
        });
        let on_input_change_member_password = ctx.link().callback(|e: Event| {
            Msg::MemberPassword(e.target_unchecked_into::<HtmlInputElement>().value())
        });
        let on_input_change_member_verifypassword = ctx.link().callback(|e: Event| {
            Msg::MemberVerifyPassword(e.target_unchecked_into::<HtmlInputElement>().value())
        });
        let on_input_change_member_mailadress = ctx.link().callback(|e: Event| {
            Msg::MemberMailAdress(e.target_unchecked_into::<HtmlInputElement>().value())
        });
        let on_input_change_member_adress = ctx.link().callback(|e: Event| {
            Msg::MemberAdress(e.target_unchecked_into::<HtmlInputElement>().value())
        });
        let on_input_change_member_birthday = ctx.link().callback(|e: Event| {
            Msg::MemberBirthday(e.target_unchecked_into::<HtmlInputElement>().value())
        });
        let on_input_change_member_phonenumber = ctx.link().callback(|e: Event| {
            Msg::MemberPhoneNumber(e.target_unchecked_into::<HtmlInputElement>().value())
        });
        html! {
            <table id="inscription">
                <section>
        <div class="container">
            <div class="formulaire">
                <form id="member_subscription_form" name="member_subscription_form_name" method="post" action="#">
                    <div>
                    <p>
                        <label for="nom"> {"Nom"}</label><br/>
                        <input onchange ={on_input_change_member_name} type="text" name="nom" id="nom" placeholder="Ex: Antoine" size="25" maxlength="100"/>
                    </p>
                    <p>
                        <label for="prenom"> {"Prénom"}</label><br/>
                        <input onchange ={on_input_change_member_surname} type="text" name="prenom" id="prenom" placeholder="Ex: Dubuisson" size="25" maxlength="100"/>
                    </p>
                    <p>
                        <label for="date_naissance"> {"Date de naissance"} </label><br/>
                        <input onchange ={on_input_change_member_birthday} type="date" name="date-naissance" id="date-naissance" placeholder="Ex: 18/12/2000" size="25" maxlength="100"/></p>
                    <p>
                        <label for="numero_tel"> {"Numero de telephone"}  </label><br/>
                        <input onchange ={on_input_change_member_phonenumber} type="tel" name="tel" id="tel" placeholder="01.02.03.04.05" size="25" maxlength="100"/>
                    </p>
                    <p>
                        <label for="adresse"> {"Adresse"}  </label><br/>
                        <input onchange ={on_input_change_member_adress} type="text" name="adresse" id="adresse" placeholder="9 rue des tuleries" size="25" maxlength="100"/>
                    </p>
                    <p>
                        <label for="adresse_mail"> {"Adresse mail"}  </label> <br/>
                        <input onchange ={on_input_change_member_mailadress} type="email" name="mail" id="mail" placeholder="Email@email.**" size="25" maxlength="100"/>
                    </p>
                    <p>
                        <label for="mot_de_passe"> {"Mot de passe"} </label> <br/>
                        <input onchange ={on_input_change_member_password} type="password" name="pass" id="pass" placeholder="*" size="25" maxlength="100"/>
                    </p>
                    <p>
                        <label for="confirmation_mp"> {"Confirmation"}  </label> <br/>
                        <input onchange ={on_input_change_member_verifypassword} type="password" name="conf" id="conf" placeholder="*" size="25" maxlength="100"/>
                    </p>
                    </div>
                    <div>
                        <button id="TpTest" type="button" onclick={ctx.link().callback(|_| Msg::GetRecordMember)}>{"Valider"}   </button>
                    </div>
                </form>
            </div>
        </div>
    </section>

            </table>
            }
    }
    fn get_html_inscrire_fin(&self, ctx: &Context<Self>) -> Html {
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
    fn get_html_la_carte(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <header>
                    <div class="banner">
                        <div class="img-banner"></div>
                        <div class="titre">
                            <h1>
                                <h1> {"Notre carte"}</h1>
                                <hr color="black"/>
                                <h2>{"Spécialité asiatique"}</h2>
                            </h1>
                        </div>
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
        html! {
            <main>
                <section id="contact.html"></section>
            </main>
        }
    }
    fn get_html_compte(&self, ctx: &Context<Self>) -> Html {
        let on_input_change_member_password = ctx.link().callback(|e: Event| {
            Msg::MemberPassword(e.target_unchecked_into::<HtmlInputElement>().value())
        });
        let on_input_change_member_mailadress = ctx.link().callback(|e: Event| {
            Msg::MemberMailAdress(e.target_unchecked_into::<HtmlInputElement>().value())
        });
        html! {
            //TODO : Modification Css
            <>
                <div class="subscribe">
                    <header>
                        <div class="banner">
                            <div class="img-banner"></div>
                            <div class="titre">
                                <h1>  {"Mon compte"} </h1>
                            </div>
                        </div>
                    </header>
                    <main>
                        <section>
                            <div class="container">
                                <div class="formulaire">
                                    <form method="post" action="#">
                                        <div>
                                            <p><label for="identifiant"> {"Identifiant"} </label><br/>
                                                <input onchange ={on_input_change_member_mailadress} type="email" name="mail" id="mail" placeholder="Votre email" size="25" maxlength="100"/>
                                            </p>
                                            <p><label for="Mp"> {"Mot de passe"} </label><br/>
                                                <input onchange ={on_input_change_member_password} type="password" name="pass" id="pass" placeholder="Votre mot de passe" size="25" maxlength="100"/>
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
        Self { value: 0, members: Vec::new(), products: Vec::new(), commande: Vec::new(), current_request: Msg::Home, product:None , member:None}
    }

    fn  update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
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

            Msg::UpdateMemberList(mem) => {
                self.members = mem;
                true
            }

            Msg::GetMembers => {
                self.current_request = Msg::GetMembers;
                console::log!("execution START of update fn / Msg::GetMembers...");
                spawn_local(
                    wrap(
                        async {
                            console::log!("execution START of Request::get(\"/api/member\")...");
                            let fetched_member = Request::get("/api/member")
                                .send()
                                .await
                                .unwrap()
                                .json()
                                .await
                                .unwrap();
                            console::log!("execution END of Request::get(\"/api/member\")...");
                            fetched_member
                        },
                        _ctx.link().callback(|fetched_member| Msg::UpdateMemberList(fetched_member)))
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

                console::log!("execution END of update fn / Msg::GetProducts ");
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

            Msg::GetMember => {
                self.current_request = Msg::GetMember;
                console::log!("execution of update fn / Msg::GetMember...");
                true
            }

            Msg::GetMemberEnd => {
                self.current_request = Msg::GetMemberEnd;
                console::log!("execution of update fn / Msg::GetMemberEnd");
                true
            }

            Msg::GetProductFrom => {
                self.current_request = Msg::GetProductFrom;
                console::log!("execution of update fn / Msg::GetProductFrom");
                true
            }
            Msg::GetRecordProduct => {
                self.current_request = Msg::GetRecordProduct;

                set_item("product",serde_json::to_string(self.product.as_ref().unwrap()).unwrap().as_str());
                console::log!("execution START of update fn / Msg::GetRecordProduct...");

                spawn_local(
                    wrap(
                        async {
                            console::log!("execution START of Request::get(\"/api/new_produit\")...");
                            let route = format!("/api/new_produit");
                            console::log!("route : {}", route.as_str());
                            let status = Request::post( route.as_str())
                                .header("Content-Type","application/json")
                                .body(
                                    /*format!(
                                        "{{\"nom_produit\":\"{}\", \"ingredients\":\"{}\", \"prix\":{}}}",
                                        get_item("nom_produit"),get_item("ingredients"), get_item("prix")
                                    )*/
                                    get_item("product")
                                )
                                .send()
                                .await
                                .unwrap()
                                .json()
                                .await
                                .unwrap();
                            console::log!("execution END of Request::get(\"/api/new_produit\")...");
                            status

                        },
                        _ctx.link().callback(|status| Msg::GetRecordProductStatus(status)))

                );
                console::log!("execution END of update fn / Msg::GetRecordProduct ");

                true
            }
            Msg::GetRecordMemberStatus(status) => {
                self.current_request = Msg::GetRecordMemberStatus(status);
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
            Msg::ProductName(product_name) => {
                let optional_pdt = self.product.as_mut();
                match optional_pdt {
                    Some(pdt) => {
                        pdt.nom_produit = product_name;
                        //set_item("nom_produit",product_name.as_str());
                        //console::log!(format!("updated nom_produit_value:{}", self.product.as_ref().unwrap().nom_produit));
                        //console::log!(format!("updated nom_produit_value in storage :{}", get_item("nom_produit")));
                        console::log!(format!("updated self.product.nom_produit value:{}", self.product.as_ref().unwrap().nom_produit));
                    }
                    _ => {
                        self.product = Some(Produit { nom_produit: product_name, ingredients: "".to_string(), prix: -1.00 });
                        console::log!(format!("created self.product.nom_produit value:{}", self.product.as_ref().unwrap().nom_produit));
                        //set_item("nom_produit",product_name.as_str());
                        //console::log!(format!("created product with nom_produit_value:{}", get_item("nom_produit")));
                    }
                }
                    true

            }

            Msg::IngredientName(ingredient_name) => {
                let optional_pdt = self.product.as_mut();
                match optional_pdt {
                    Some(pdt) => {
                        pdt.ingredients = ingredient_name;
                        //set_item("nom_produit",product_name.as_str());
                        //console::log!(format!("updated nom_produit_value:{}", self.product.as_ref().unwrap().nom_produit));
                        //console::log!(format!("updated nom_produit_value in storage :{}", get_item("nom_produit")));
                        console::log!(format!("updated self.product.ingredients value:{}", self.product.as_ref().unwrap().ingredients));
                    }
                    _ => {
                        self.product = Some(Produit { nom_produit: "".to_string(), ingredients: ingredient_name, prix: -1.00 });
                        console::log!(format!("created self.product.ingredients value:{}", self.product.as_ref().unwrap().ingredients));
                        //set_item("nom_produit",product_name.as_str());
                        //console::log!(format!("created product with nom_produit_value:{}", get_item("nom_produit")));
                    }
                }
                true
            }

            Msg::PriceNumber(price_number) => {
                let optional_pdt = self.product.as_mut();
                match optional_pdt {
                    Some(pdt) => {
                        pdt.prix = price_number;
                        //set_item("nom_produit",product_name.as_str());
                        //console::log!(format!("updated nom_produit_value:{}", self.product.as_ref().unwrap().nom_produit));
                        //console::log!(format!("updated nom_produit_value in storage :{}", get_item("nom_produit")));
                        console::log!(format!("updated self.product.prix value:{}", self.product.as_ref().unwrap().prix));
                    }
                    _ => {
                        self.product = Some(Produit { nom_produit: "".to_string(), ingredients: "".to_string(), prix: price_number });
                        console::log!(format!("created self.product.prix value:{}", self.product.as_ref().unwrap().prix));
                        //set_item("nom_produit",product_name.as_str());
                        //console::log!(format!("created product with nom_produit_value:{}", get_item("nom_produit")));
                    }
                }
                true
            }
            Msg::GetRecordMember => {
                self.current_request = Msg::GetRecordMember;
                set_item("member",serde_json::to_string(self.member.as_ref().unwrap()).unwrap().as_str());
                console::log!("execution START of update fn / Msg::GetRecordMember...");

                spawn_local(
                    wrap(
                        async {
                            console::log!("execution START of Request::get(\"/api/new_member\")...");
                            let route = format!("/api/new_member");
                            console::log!("route : {}", route.as_str());
                            let status = Request::post( route.as_str())
                                .header("Content-Type","application/json")
                                .body(
                                    get_item("member")
                                )
                                .send()
                                .await
                                .unwrap()
                                .json()
                                .await
                                .unwrap();
                            console::log!("execution END of Request::get(\"/api/new_member\")...");
                            status

                        },
                        _ctx.link().callback(|status| Msg::GetRecordMemberStatus(status)))

                );
                console::log!("execution END of update fn / Msg::GetRecordMember ");
                true

            }
            Msg::GetRecordProductStatus(status) => {
                self.current_request = Msg::GetRecordProductStatus(status);
                true
            }
            Msg::MemberName(member_name) => {
                let optional_mem = self.member.as_mut();
                match optional_mem {
                    Some(pdt) => {
                        pdt.nom = member_name;
                        console::log!(format!("updated self.member.nom value:{}", self.member.as_ref().unwrap().nom));
                    }
                    _ => {
                        self.member = Some(Member { nom: member_name, prenom: "".to_string(),date_naissance: "".to_string(),numero_tel: "".to_string(),adresse_mail: "".to_string(),mot_de_passe: "".to_string(),confirmation_mp: "".to_string(),adresse: "".to_string()});

                        console::log!(format!("created self.member.nom value:{}", self.member.as_ref().unwrap().nom));
                    }
                }
                true

            }
            Msg::MemberSurname(member_att) => {
                let optional_mem = self.member.as_mut();
                match optional_mem {
                    Some(pdt) => {
                        pdt.prenom = member_att;
                        console::log!(format!("updated self.member.nom value:{}", self.member.as_ref().unwrap().prenom));
                    }
                    _ => {
                        self.member = Some(Member { nom: "".to_string(), prenom: member_att,date_naissance: "".to_string(),numero_tel: "".to_string(),adresse_mail: "".to_string(),mot_de_passe: "".to_string(),confirmation_mp: "".to_string(),adresse: "".to_string()});

                        console::log!(format!("created self.member.nom value:{}", self.member.as_ref().unwrap().prenom));
                    }
                }
                true

            }
            Msg::MemberBirthday(member_att) => {
                let optional_mem = self.member.as_mut();
                match optional_mem {
                    Some(pdt) => {
                        pdt.date_naissance = member_att;
                        console::log!(format!("updated self.member.nom value:{}", self.member.as_ref().unwrap().date_naissance));
                    }
                    _ => {
                        self.member = Some(Member { nom: "".to_string(), prenom: "".to_string(),date_naissance: member_att,numero_tel: "".to_string(),adresse_mail: "".to_string(),mot_de_passe: "".to_string(),confirmation_mp: "".to_string(),adresse: "".to_string()});

                        console::log!(format!("created self.member.nom value:{}", self.member.as_ref().unwrap().date_naissance));
                    }
                }
                true

            }
            Msg::MemberPhoneNumber(member_att) => {
                let optional_mem = self.member.as_mut();
                match optional_mem {
                    Some(pdt) => {
                        pdt.numero_tel = member_att;
                        console::log!(format!("updated self.member.nom value:{}", self.member.as_ref().unwrap().numero_tel));
                    }
                    _ => {
                        self.member = Some(Member { nom: "".to_string(), prenom: "".to_string(),date_naissance: "".to_string(),numero_tel: member_att,adresse_mail: "".to_string(),mot_de_passe: "".to_string(),confirmation_mp: "".to_string(),adresse: "".to_string()});

                        console::log!(format!("created self.member.nom value:{}", self.member.as_ref().unwrap().numero_tel));
                    }
                }
                true

            }
            Msg::MemberMailAdress(member_att) => {
                let optional_mem = self.member.as_mut();
                match optional_mem {
                    Some(pdt) => {
                        pdt.adresse_mail = member_att;
                        console::log!(format!("updated self.member.nom value:{}", self.member.as_ref().unwrap().adresse_mail));
                    }
                    _ => {
                        self.member = Some(Member { nom: "".to_string(), prenom: "".to_string(),date_naissance: "".to_string(),numero_tel: "".to_string(),adresse_mail: member_att,mot_de_passe: "".to_string(),confirmation_mp: "".to_string(),adresse: "".to_string()});
                        console::log!(format!("created self.member.nom value:{}", self.member.as_ref().unwrap().adresse_mail));
                    }
                }
                true

            }
            Msg::MemberPassword(member_att) => {
                let optional_mem = self.member.as_mut();
                match optional_mem {
                    Some(pdt) => {
                        pdt.mot_de_passe = member_att;
                        console::log!(format!("updated self.member.nom value:{}", self.member.as_ref().unwrap().mot_de_passe));
                    }
                    _ => {
                        self.member = Some(Member { nom: "".to_string(), prenom: "".to_string(),date_naissance: "".to_string(),numero_tel: "".to_string(),adresse_mail: "".to_string(),mot_de_passe: member_att,confirmation_mp: "".to_string(),adresse: "".to_string()});
                        console::log!(format!("created self.member.nom value:{}", self.member.as_ref().unwrap().mot_de_passe));
                    }
                }
                true

            }
            Msg::MemberVerifyPassword(member_att) => {
                let optional_mem = self.member.as_mut();
                match optional_mem {
                    Some(pdt) => {
                        pdt.confirmation_mp = member_att;
                        console::log!(format!("updated self.member.nom value:{}", self.member.as_ref().unwrap().confirmation_mp));
                    }
                    _ => {
                        self.member = Some(Member { nom: "".to_string(), prenom: "".to_string(),date_naissance: "".to_string(),numero_tel: "".to_string(),adresse_mail: "".to_string(),mot_de_passe: "".to_string(),confirmation_mp: member_att,adresse: "".to_string()});
                        console::log!(format!("created self.member.nom value:{}", self.member.as_ref().unwrap().confirmation_mp));
                    }
                }
                true

            }
            Msg::MemberAdress(member_att) => {
                let optional_mem = self.member.as_mut();
                match optional_mem {
                    Some(pdt) => {
                        pdt.adresse = member_att;
                        console::log!(format!("updated self.member.nom value:{}", self.member.as_ref().unwrap().adresse));
                    }
                    _ => {
                        self.member = Some(Member { nom: "".to_string(), prenom: "".to_string(),date_naissance: "".to_string(),numero_tel: "".to_string(),adresse_mail: "".to_string(),mot_de_passe: "".to_string(),confirmation_mp: "".to_string(),adresse: member_att});
                        console::log!(format!("created self.member.nom value:{}", self.member.as_ref().unwrap().adresse));
                    }
                }
                true
            }
            Msg::UpdateLoginMember(mem) => {
                self.members = mem;
                true
            }

            Msg::GetLoginMember => {
                self.current_request = Msg::GetLoginMember;
                console::log!("execution START of update fn / Msg::GetLoginMember...");
                spawn_local(
                    wrap(
                        async {
                            console::log!("execution START of Request::get(\"/api/login_member\")...");
                            let fetched_login_member = Request::get("/api/login_member")
                                .send()
                                .await
                                .unwrap()
                                .json()
                                .await
                                .unwrap();
                            console::log!("execution END of Request::get(\"/api/login_member\")...");
                            fetched_login_member
                        },
                        _ctx.link().callback(|fetched_login_member| Msg::UpdateLoginMember(fetched_login_member)))
                );

                console::log!("execution END of update fn / Msg::GetLoginMember ");
                true
            }
            _ => { true }
        }
    }


    fn view(&self, ctx: &Context<Self>) -> Html {
        let videos = self.get_html_member_list(ctx);
        let products = self.get_html_product_list(ctx);
        let commande = self.get_html_cmd_list(ctx);
        let compte = self.get_html_compte(ctx);
        let la_carte = self.get_html_la_carte(ctx);
        let contact = self.get_html_contact(ctx);
        let accueil = self.get_html_accueil(ctx);
        let navbar = self.get_html_nav(ctx);
        let faq = self.get_html_faq(ctx);
        let footer = self.get_html_footer(ctx);
        let inscrire = self.get_html_inscrire(ctx);
        let inscrire_fin = self.get_html_inscrire_fin(ctx);
        let product_form = self.get_html_product_form(ctx);
        let main_view_content = match self.current_request {
            Msg::GetMembers => {
                videos
            }
            Msg::GetProducts => {
                products
            }
            Msg::GetCommande => {
                commande
            }
            Msg::GetMember => {
                inscrire
            }
            Msg::GetMemberEnd => {
                inscrire_fin
            }
            Msg::GetProductFrom => {
                product_form
            }
            Msg::GetHome => {
                accueil
            }
            Msg::GetFAQ => {
                faq
            }
            Msg::GetAnnonce => {
                accueil
            }
            Msg::GetCompte => {
                compte
            }
            Msg::GetLaCarte => {
                la_carte
            }
            Msg::GetContact => {
                accueil
            }
            _ => { accueil }
        };

        html! {
            <>
                {navbar}
                <main>
                    {main_view_content}
                </main>
                {footer}
            </>
            }
        }
}


fn main() {
    yew::start_app::<App>();
}