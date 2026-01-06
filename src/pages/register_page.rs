use dioxus::prelude::{component, rsx, use_navigator, Element};

#[component]
pub fn RegisterPage() -> Element {
    let nav = use_navigator();

    // halaman register di bagian tengah
    rsx! {
                    section {
                        class :  "rounded-md shadow-md p-2 mx-auto ",

                           form { class: "flex flex-col items-center justify-center w-full gap-6",
                                // Username Field
                                label { class: "w-full flex flex-col gap-2 font-semibold text-[#333232] text-[0.9rem]",
                                    "Username"
                                    input {
                                        r#type: "text",
                                        class: "w-full py-4 px-5 border-2 border-[#d0d0d0] rounded-[12px] text-base bg-[#f8f8f8] transition-all duration-300 box-border outline-none text-[#333232]",

                                        placeholder: "Input your username",
                                        value: "{hande_data.read().username}",
                                        // oninput: move |_| {
                                        //     println!("")
                                        // },
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
                                        // oninput: move |e| {
                                        //     hande_data.write().email = e.value().clone();
                                        // },
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
                                        // oninput: move |e| {
                                        //     hande_data.write().password = e.value().clone();
                                        // },
                                    }
                                }

            button {
        r#type : "submit",

                "Submit"

            }
    }
            },

                    nav {

                        button {
                        r#type : "button",
                        class : "cursor-pointer border-2 p-3",
                        onclick: move |_| {
                            nav.go_back();
                        },
                        text : "Back"
                    }
                }

                }
}
