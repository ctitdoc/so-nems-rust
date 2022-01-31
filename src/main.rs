use gloo_console as console;
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
          <img src="img/nems-logo.jpg" alt=""/>
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
</div>
    }
    }
}

fn main() {
    yew::start_app::<App>();
}