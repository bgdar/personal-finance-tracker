use dioxus::prelude::*;
use crate::components::styling::STYLING;
use crate::components::route::Route;

#[component]
pub fn Dashboard() -> Element {
    let styling_body = "
        width: 80vw;
        max-width: 800px;
        height: 80vh;
        margin: 0 auto;
        box-sizing: border-box;
        background: linear-gradient(to right, #c0c0c0, #ffffff);
        position: fixed;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        z-index: 20;padding: 2rem; border: none;border-radius: 30px;box-shadow: 2px 2px 12px rgba(0, 0, 0, 0.2);text-align: center;display: flex;flex-direction: column;justify-content: center;align-items: center";

    let button_container_style = "margin-top: 2rem;display: flex;justify-content: center;align-items: center;width: 100%;";

    rsx! {
        section { style: styling_body,
            h2 { style: "
                    margin: 0 0 1rem 0;
                    font-size: 2.5rem;
                    font-weight: bold;
                    color: #333;
                    text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.1);
                ",
                "Money Word"
            }

            div { style: button_container_style,
                Link { to: Route::LoginPage, style: STYLING.button_sy, "Login" }
            }
        }
    }
}

#[component]
pub fn HomePage()->Element{


    rsx!{

        h2 { "Home page" }
    }
       
}