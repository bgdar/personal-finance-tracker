use dioxus:: prelude::*;
use dioxus_router::prelude::*;

mod components;
mod pages;
mod user;

use components::route::Route;


fn main() {
   
    dioxus::launch(App);
    
    
}
   


#[component]
fn App() -> Element {

    rsx!{
        Router::<Route> {
        }
    }
}
