use dioxus:: prelude::*;
// use dioxus_router::prelude::*;

mod components;
mod pages;
mod database;
mod server;

use server::route::Route;

fn main() {
   
    dioxus::launch(app);
        
}

#[component]
fn app() -> Element {

    rsx!{
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        Router::<Route> {
        }
    }
}
