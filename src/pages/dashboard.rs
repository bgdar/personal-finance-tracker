
use dioxus::prelude::*;

use crate::components::route::Route;

#[component]
pub fn Dashboard() -> Element {
rsx! {
    section { class: "
            w-[95vw] max-w-[1200px] h-auto min-h-[85vh] mx-auto box-border 
            bg-gradient-to-r from-[#c0c0c0] to-[#ffffff] 
            fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 
            z-20 p-6 md:p-10 border-none rounded-[30px] 
            shadow-[2px_2px_12px_rgba(0,0,0,0.2)] 
            flex flex-col gap-6
        ",

        // Header Dashboard
        header { class: "flex flex-col md:flex-row md:items-center md:justify-between gap-4",
            h2 { class: "text-2xl font-bold text-gray-800", "Crypto Dashboard" }
            div { class: "bg-[#1c1c1c] text-white px-4 py-2 rounded-full shadow", "Welcome, User123" }
        }

        // Konten Dashboard
        main { class: "grid grid-cols-1 md:grid-cols-2 gap-6 w-full",

            // Card Balance
            div { class: "bg-white p-6 rounded-2xl shadow flex flex-col justify-between",
                h3 { class: "text-lg font-semibold text-gray-700", "Total Balance" }
                p { class: "text-3xl font-bold text-green-600", "$ 12,450.00" }
            }

            // Card Recent Transactions
            div { class: "bg-white p-6 rounded-2xl shadow flex flex-col",
                h3 { class: "text-lg font-semibold text-gray-700", "Recent Transactions" }
                ul { class: "mt-4 space-y-2 text-gray-600",
                    li { "+ $250 from Alice" }
                    li { "- $50 to Bob" }
                    li { "+ $1,200 from Binance" }
                }
            }
        }

        // Footer / Navigation
        footer { class: "mt-auto flex justify-center md:justify-end",
            Link {
                to: Route::Home {},
                class: "bg-[#1c1c1c] text-white px-6 py-2 rounded-full shadow hover:bg-gray-800 transition-colors duration-300",
                "home"
            }
        }
    }
}
    

}