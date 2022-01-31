/*mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, HtmlElement, MouseEvent};


/* Franck commented :

use error_chain::error_chain;
use std::io::Read;

End Franck commented */

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}


#[wasm_bindgen]
pub fn greet() {

       // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_text_content(Some(&msg));

    my_form.append_child(&msg)?;

    Ok(());


    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let my_element = document.get_element_by_id("member_subscription_form").unwrap();
    let my_form: web_sys::HtmlFormElement = my_element.dyn_into::<web_sys::HtmlFormElement>().unwrap();
    let controls = my_form.elements();
    let count = controls.length();
    //INPUT NOM
    let my_input_element_nom = controls.item(0).unwrap();
    let my_nom_text_input: web_sys::HtmlInputElement = my_input_element_nom.dyn_into::<web_sys::HtmlInputElement>().unwrap();
    let nom_value = my_nom_text_input.value();
    //INPUT PRENOM
    let my_input_element_prenom = controls.item(1).unwrap();
    let my_prenom_text_input: web_sys::HtmlInputElement = my_input_element_prenom.dyn_into::<web_sys::HtmlInputElement>().unwrap();
    let prenom_value = my_prenom_text_input.value();
    //INPUT DATE
    let my_input_element_date = controls.item(2).unwrap();
    let my_date_text_input: web_sys::HtmlInputElement = my_input_element_date.dyn_into::<web_sys::HtmlInputElement>().unwrap();
    let date_value = my_date_text_input.value();
    //INPUT NUM TEL
    let my_input_element_tel = controls.item(3).unwrap();
    let my_tel_text_input: web_sys::HtmlInputElement = my_input_element_tel.dyn_into::<web_sys::HtmlInputElement>().unwrap();
    let tel_value = my_tel_text_input.value();
    //INPUT MAIL
    let my_input_element_mail = controls.item(4).unwrap();
    let my_mail_text_input: web_sys::HtmlInputElement = my_input_element_mail.dyn_into::<web_sys::HtmlInputElement>().unwrap();
    let mail_value = my_mail_text_input.value();
    //INPUT PASSWORD
    let mp_value = "confidntial password";

    let msg = format!(
     "le nom est {},
     le prenom est {},
     la date de naissance est {},
     le numéro de téléphone est {},
     l'adresse mail est {},
     le mot de passe est {}", nom_value, prenom_value, date_value, tel_value, mail_value, mp_value);



    //let my_form_data = web_sys::FormData::new_with_form(my_form);
    /*    for field in my_form_data.keys()  {
            alert(field);
        }*/

    alert(&my_form.name());
    alert(&count.to_string());
    alert(&nom_value);
    alert(&prenom_value);
    alert(&date_value);
    alert(&tel_value);
    alert(&mail_value);
    alert(&mp_value);
    alert(&msg);


    let val = document.create_element("p").unwrap();
    val.set_text_content(Some(&msg));

    my_form.append_child(&val);
}

#[wasm_bindgen]
pub fn test_dom() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");


    let val = document.create_element("p")?;
    val.set_text_content(Some("Hello from Rust!"));

    body.append_child(&val)?;

    Ok(())
}



#[wasm_bindgen]
error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

pub fn test_commande() -> Result<()> {
let mut res = reqwest::blocking::get("localhost:8000/member")?;
let mut body = String::new();
res.read_to_string(&mut body)?;

println!("Status: {}", res.status());
println!("Headers:\n{:#?}", res.headers());
println!("Body:\n{}", body);

Ok(())

}

////////////////////////////////////////////////////////////////////////////////////////////////////
//test-yew counter
////////////////////////////////////////////////////////////////////////////////////////////////////

*/