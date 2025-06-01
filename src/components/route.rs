use dioxus::prelude::*;
use crate::pages::pages::*;
use crate::user::user::*; 


#[derive(Clone,Routable)]
pub enum Route  {
    #[route("/")]
    Dashboard,
    #[route("/login")]
    LoginPage,
    
    #[route("/home")]
    HomePage,


    // #[route("/about")]
    // AboutPage,
}

