use dioxus::{html::u::text_size_adjust, prelude::*};

use crate::components::route::Route;

struct HandleData {
    email: String,
    password: String,
    username: String,
}

/// Fungsi untuk menangani data input dari form login
/// sekarang ini belu di validasi ke database hanya membandingkan dengan username : 'dar' dan password 'dar'
#[component]
pub fn LoginPage() -> Element {
    let mut hande_data = use_signal(|| HandleData {
        email: String::new(),
        password: String::new(),
        username: String::new(),
    });
    // untuk
    let mut info_popup = use_signal(|| String::from(""));

    let nav = use_navigator();

    let tets_username: &str = "dar";
    let tets_password: &str = "dar";
    let tes_email: &str = "dar@gamil.com";

    rsx! {
        section { class: "flex justify-center items-center fixed top-0 left-0 w-screen h-screen bg-[linear-gradient(135deg,#e8e8e8_0%,#f5f5f5_50%,#d3d3d3_100%)] z-[100]",

            // Back Button
            nav {
                button {
                    r#type: "button",
                    class: "absolute top-4 left-4 px-4 py-2 bg-[rgba(51,50,50,0.9)] text-white border-none rounded-lg text-[0.9rem] cursor-pointer transition-all duration-300 backdrop-blur-[5px]",
                    onclick: move |_| {
                        nav.go_back();
                    },
                    "← Back"
                }
            }
            // poppup
            {
                if !info_popup.read().is_empty() {
                    rsx! {
                        div { class: "text-center border-2 shadow-lg rounded-b-lg p-2 z-20  ",
                            p { {info_popup} }
                        }
                    }
                } else {
                    rsx! {}
                }
            }

            // Main Form Container
            div { class: "bg-[rgba(255,255,255,0.98)] rounded-[24px] py-12 px-10 shadow-[0_20px_40px_rgba(51,50,50,0.15)] backdrop-blur-[10px] border border-[rgba(200,200,200,0.3)] w-full max-w-[400px] relative",

                h2 { class: "text-[1.8rem] font-bold text-[#333232] mb-4 text-center",
                    "Login to Your Wallet"
                }

                form { class: "flex flex-col items-center justify-center w-full gap-6",
                    // Username Field
                    label { class: "w-full flex flex-col gap-2 font-semibold text-[#333232] text-[0.9rem]",
                        "Username"
                        input {
                            r#type: "text",
                            class: "w-full py-4 px-5 border-2 border-[#d0d0d0] rounded-[12px] text-base bg-[#f8f8f8] transition-all duration-300 box-border outline-none text-[#333232]",

                            placeholder: "Input your username",
                            value: "{hande_data.read().username}",
                            oninput: move |e| {
                                hande_data.write().username = e.value().clone();
                            },
                        }
                    }

                    // Email Field
                    label { class: "w-full flex flex-col gap-2 font-semibold text-[#333232] text-[0.9rem]",
                        "Email"
                        input {
                            r#type: "email",
                            class: "w-full py-4 px-5 border-2 border-[#d0d0d0] rounded-[12px] text-base bg-[#f8f8f8] transition-all duration-300 box-border outline-none text-[#333232]",

                            placeholder: "Input your email",
                            value: "{hande_data.read().email}",
                            oninput: move |e| {
                                hande_data.write().email = e.value().clone();
                            },
                        }
                    }

                    // Password Field
                    label { class: "w-full flex flex-col gap-2 font-semibold text-[#333232] text-[0.9rem]",
                        "Password"
                        input {
                            r#type: "password",
                            class: "w-full py-4 px-5 border-2 border-[#d0d0d0] rounded-[12px] text-base bg-[#f8f8f8] transition-all duration-300 box-border outline-none text-[#333232]",

                            placeholder: "Input your password",
                            value: "{hande_data.read().password}",
                            oninput: move |e| {
                                hande_data.write().password = e.value().clone();
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
                        a { class: "text-[#666666] text-[0.85rem] no-underline cursor-pointer transition-colors duration-300",
                            "Requirement Password"
                        }
                        a { class: "text-[#333232] text-[0.9rem] no-underline cursor-pointer font-semibold transition-colors duration-300",
                            "sign up"
                        }
                    }

                    // Submit Button
                    button {
                        r#type: "submit",
                        class: "w-full py-4 px-8 bg-[linear-gradient(135deg,#333232_0%,#555555_50%,#333232_100%)] text-white border-none rounded-[12px] text-base font-semibold cursor-pointer transition-all duration-300 shadow-[0_4px_15px_rgba(51,50,50,0.4)] mt-2",

                        onclick: move |_| {
                            if &hande_data.read().username == tets_username
                                && &hande_data.read().password == tets_password
                                && &hande_data.read().email == tes_email
                            {
                                nav.replace(Route::Home {});
                            } else {
                                info_popup.set("ada yang salah saat login".to_string());
                            }
                        },
                        "Submit"
                    }

                    // Divider
                    div { class: "flex items-center w-full my-4 text-[#666666] text-[0.9rem]",

                        div { class: "flex-1 h-px bg-[#cccccc] mx-4" }
                        span { "or" }
                        div { class: "flex-1 h-px bg-[#cccccc] mx-4" }
                    }

                    // Social Login Buttons
                    div { class: "flex justify-center gap-4 w-full",
                        // Google Button
                        button {
                            class: "w-[50px] h-[50px] rounded-full border-2 border-[#d0d0d0] bg-white flex items-center justify-center cursor-pointer transition-all duration-300 shadow-[0_2px_8px_rgba(51,50,50,0.1)]",

                            onclick: move |_| {
                                println!("Google login clicked");
                            },
                            "G" // Ganti dengan icon Google jika tersedia
                        }

                        // Facebook Button
                        button {
                            class: "w-[50px] h-[50px] rounded-full border-2 border-[#d0d0d0] bg-white flex items-center justify-center cursor-pointer transition-all duration-300 shadow-[0_2px_8px_rgba(51,50,50,0.1)]",

                            onclick: move |_| {
                                println!("Facebook login clicked");
                            },
                            "f" // Ganti dengan icon Facebook jika tersedia
                        }

                        // Apple Button
                        button {
                            class: "w-[50px] h-[50px] rounded-full border-2 border-[#d0d0d0] bg-white flex items-center justify-center cursor-pointer transition-all duration-300 shadow-[0_2px_8px_rgba(51,50,50,0.1)]",

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
