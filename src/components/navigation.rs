
use dioxus::prelude::*;
use dioxus_free_icons::{Icon,
icons::io_icons::IoHome,
     icons::fa_solid_icons::{FaCreditCard, FaStore, FaAddressCard, FaUser}};



#[component]
pub fn Navigation() -> Element {

    rsx! {
        nav {
            // Mobile: posisi bawah, horizontal
            // >=800px: posisi kiri, vertical
            class: "
                fixed
                bg-[#1c1c1c]
                shadow-lg
                max-[800px]:bottom-4 max-[800px]:left-1/2 max-[800px]:transform max-[800px]:-translate-x-1/2 max-[800px]:flex-row max-[800px]:rounded-full max-[800px]:px-4 max-[800px]:py-2 max-[800px]:w-[90%] max-[800px]:max-w-md max-[800px]:justify-around
                min-[800px]:top-1/2 min-[800px]:left-4 min-[800px]:-translate-y-1/2 min-[800px]:flex-col min-[800px]:rounded-3xl min-[800px]:px-3 min-[800px]:py-6 min-[800px]:items-center min-[800px]:gap-4
                flex items-center
            ",

            // Icon kiri
            div { class: "bg-white rounded-full p-2",
                Icon {
                    icon: FaCreditCard,
                    width: 20,
                    height: 20,
                    fill: "black",
                }
            }
            div { class: "bg-white rounded-full p-2",
                Icon {
                    icon: FaStore,
                    width: 20,
                    height: 20,
                    fill: "black",
                }
            }

            // Icon Home besar
            div { class: "relative max-[800px]:-mt-8 min-[800px]:my-4 bg-[#1c1c1c] rounded-full p-3 border-4 border-[#d9d9d9]",
                Icon {
                    icon: IoHome,
                    width: 28,
                    height: 28,
                    fill: "white",
                }
            }

            // Icon kanan
            div { class: "bg-white rounded-full p-2",
                Icon {
                    icon: FaAddressCard,
                    width: 20,
                    height: 20,
                    fill: "black",
                }
            }
            div { class: "bg-white rounded-full p-2",
                Icon {
                    icon: FaUser,
                    width: 20,
                    height: 20,
                    fill: "black",
                }
            }
        }
    }
}