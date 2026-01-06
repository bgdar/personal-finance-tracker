use dioxus::prelude::*;
use dioxus_free_icons::{Icon ,icons::fa_solid_icons::{FaUser, FaBitcoinSign}};

use  crate::components::navigation::Navigation;

#[component]
pub fn Wallet()->Element { 

    rsx!{
        main { class: "h-screen w-screen bg-[#d9d9d9] relative",
            // header bagian atas
            div { id: "header", class: "flex",
                div { class: "rounded bg-[#373737] text-white p-2 m-2",
                    Icon {
                        icon: FaUser,
                        width: 20,
                        height: 20,
                        fill: "currentColor",
                    }
                }
                div {
                    h3 { "nama pengguna" }
                    p { "id:1212232232" }
                }
            }
            // konten utama
            div { class: "flex items-center justify-center",
                p { "Wallet content goes here" }
            }
        }
        // navigasi
        Navigation {}
    }
}