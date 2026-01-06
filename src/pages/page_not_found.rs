use dioxus::{ prelude::*};



#[component]
pub fn PageNotFound()->Element {
    rsx!{
        h3 { class: "text-center ",
            text { "halaman tidak di temukan" }
        }
        
    }
}