use dioxus::prelude::*;
use crate::pages::home::Home;
use crate::pages::dashboard::Dashboard;
use crate::pages::login_page::LoginPage; 
use crate::pages::wallet::Wallet;


#[derive(Clone,Routable)]
pub enum Route  {
    #[route("/")]
    Dashboard{},
    
    // main page
    #[route("/home")]
    Home{},
    #[route("/wallet")]
    Wallet{},
    // User    
    #[route("/login")]
    LoginPage{},

    // #[route("/about")]
    // AboutPage,
}

