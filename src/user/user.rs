
use dioxus::{ html::{nav, u::text_size_adjust}, prelude::*};
use crate::components::{route::Route, styling::STYLING};


 struct HandleData {
    email: String,
    password: String,
    username:String ,
 }

 /// Fungsi untuk menangani data input dari form login
 /// sekarang ini belu di validasi ke database hanya membandingkan dengan username : 'dar' dan password 'dar' 
#[component]
pub fn LoginPage()->Element  {

  let mut  handeData  = use_signal(|| HandleData {
        email: String::new(),
        password: String::new(),
        username : String::new(),
    });

    let nav = use_navigator();


    let tets_username : &str = "dar";
    let tets_password : &str = "dar";

    let main_styling = "display: flex; justify-content: center; align-items: center; 
flex-direction: column;
min-height: 100vh;
        background: linear-gradient(135deg, #e8e8e8 0%, #f5f5f5 50%, #d3d3d3 100%);
padding: 2rem;
    ";

    let form_container_style = "
 background: rgba(255, 255, 255, 0.98);
border-radius: 24px;
padding: 3rem 2.5rem;
        box-shadow: 0 20px 40px rgba(51, 50, 50, 0.15);
        backdrop-filter: blur(10px);
        border: 1px solid rgba(200, 200, 200, 0.3);
        width: 100%;
        max-width: 400px;
        position: relative;
    ";

    let form_style = "
        display: flex; 
        flex-direction: column; 
        align-items: center; 
        justify-content: center; 
        width: 100%;
        gap: 1.5rem;
    ";

    let title_style = "
        font-size: 1.8rem;
        font-weight: 700;
        color: #333232;
        margin-bottom: 1rem;
        text-align: center;
    ";

    let label_style = "
        width: 100%;
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
        font-weight: 600;
        color: #333232;
        font-size: 0.9rem;
    ";

    let input_style = "
        width: 100%;
        padding: 1rem 1.25rem;
        border: 2px solid #d0d0d0;
        border-radius: 12px;
        font-size: 1rem;
        background: #f8f8f8;
        transition: all 0.3s ease;
        box-sizing: border-box;
        outline: none;
        color: #333232;
    ";

    let button_primary_style = "
        width: 100%;
        padding: 1rem 2rem;
        background: linear-gradient(135deg, #333232 0%, #555555 50%, #333232 100%);
        color: white;
        border: none;
        border-radius: 12px;
        font-size: 1rem;
        font-weight: 600;
        cursor: pointer;
        transition: all 0.3s ease;
        box-shadow: 0 4px 15px rgba(51, 50, 50, 0.4);
        margin-top: 0.5rem;
    ";

    let back_button_style = "
        position: absolute;
        top: 1rem;
        left: 1rem;
        padding: 0.5rem 1rem;
        background: rgba(51, 50, 50, 0.9);
        color: white;
        border: none;
        border-radius: 8px;
        font-size: 0.9rem;
        cursor: pointer;
        transition: all 0.3s ease;
        backdrop-filter: blur(5px);
    ";

    let divider_style = "
        display: flex;
        align-items: center;
        width: 100%;
        margin: 1rem 0;
        color: #666666;
        font-size: 0.9rem;
    ";

    let divider_line_style = "
        flex: 1;
        height: 1px;
        background: #cccccc;
        margin: 0 1rem;
    ";

    let social_buttons_style = "
        display: flex;
        justify-content: center;
        gap: 1rem;
        width: 100%;
    ";

    let social_button_style = "
        width: 50px;
        height: 50px;
        border-radius: 50%;
        border: 2px solid #d0d0d0;
        background: white;
        display: flex;
        align-items: center;
        justify-content: center;
        cursor: pointer;
        transition: all 0.3s ease;
        box-shadow: 0 2px 8px rgba(51, 50, 50, 0.1);
    ";

    let requirement_link_style = "
        color: #666666;
        font-size: 0.85rem;
        text-decoration: none;
        cursor: pointer;
        transition: color 0.3s ease;
    ";

    let signup_link_style = "
        color: #333232;
        font-size: 0.9rem;
        text-decoration: none;
        cursor: pointer;
        font-weight: 600;
        transition: color 0.3s ease;
    ";

    rsx! {
        section { style: "
                display: flex; 
                justify-content: center; 
                align-items: center; 
                position: fixed; 
                top: 0; 
                left: 0; 
                width: 100vw; 
                height: 100vh; 
                background: linear-gradient(135deg, #e8e8e8 0%, #f5f5f5 50%, #d3d3d3 100%);
                z-index: 100;
            ",
            // Back Button
            nav {
                button {
                    r#type: "button",
                    style: back_button_style,
                    onclick: move |_| {
                        nav.go_back();
                    },
                    "← Back"
                }
            }

            // Main Form Container
            div { style: form_container_style,
                h2 { style: title_style, "Login to Your Wallet" }

                form { style: form_style,
                    // Username Field
                    label { style: label_style,
                        "Username"
                        input {
                            r#type: "text",
                            style: input_style,
                            placeholder: "Input your username",
                            value: "{handeData.read().username}",
                            oninput: move |e| {
                                handeData.write().username = e.value().clone();
                            },
                        }
                    }

                    // Email Field
                    label { style: label_style,
                        "Email"
                        input {
                            r#type: "email",
                            style: input_style,
                            placeholder: "Input your email",
                            value: "{handeData.read().email}",
                            oninput: move |e| {
                                handeData.write().email = e.value().clone();
                            },
                        }
                    }

                    // Password Field
                    label { style: label_style,
                        "Password"
                        input {
                            r#type: "password",
                            style: input_style,
                            placeholder: "Input your password",
                            value: "{handeData.read().password}",
                            oninput: move |e| {
                                handeData.write().password = e.value().clone();
                            },
                        }
                    }

                    // Requirement Password & Sign Up Links
                    div { style: "
                            display: flex;
                            justify-content: space-between;
                            width: 100%;
                            margin-top: -0.5rem;
                            margin-bottom: 0.5rem;
                        ",
                        a { style: requirement_link_style, "Requirement Password" }
                        a { style: signup_link_style, "sign up" }
                    }

                    // Submit Button
                    button {
                        r#type: "submit",
                        style: button_primary_style,
                        onclick: move |_| {
                            println!(
                                "Username: {}, Email: {}, Password: {}",
                                handeData.read().username,
                                handeData.read().email,
                                handeData.read().password,
                            );
                            if handeData.read().username == tets_username
                                && handeData.read().password == tets_password
                            {
                                println!("Login successful");
                                nav.replace(Route::HomePage);
                            } else {
                                println!("Login failed");
                            }
                        },
                        "Submit"
                    }

                    // Divider
                    div { style: divider_style,
                        div { style: divider_line_style }
                        span { "or" }
                        div { style: divider_line_style }
                    }

                    // Social Login Buttons
                    div { style: social_buttons_style,
                        // Google Button
                        button {
                            style: social_button_style,
                            onclick: move |_| {
                                println!("Google login clicked");
                            },
                            "G" // Ganti dengan icon Google jika tersedia
                        }

                        // Facebook Button
                        button {
                            style: social_button_style,
                            onclick: move |_| {
                                println!("Facebook login clicked");
                            },
                            "f" // Ganti dengan icon Facebook jika tersedia
                        }

                        // Apple Button
                        button {
                            style: social_button_style,
                            onclick: move |_| {
                                println!("Apple login clicked");
                            },
                            "🍎"
                        }
                    }
                }
            }
        }
    }

    }
    


//     #[component]
// pub fn SingUpPage ()-> Element {

// }