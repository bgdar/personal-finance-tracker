use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::bs_icons::{BsArrowBarDown, BsTrashFill},
    icons::fa_solid_icons::{FaBitcoinSign, FaUser},
    Icon,
};

//component
use crate::components::navigation::Navigation;

#[derive(Clone, PartialEq)]
enum MenuIcon {
    Transfer,
    Withdraw,
}

#[derive(Clone, PartialEq)]
struct MenuType {
    name: String,
    icon: MenuIcon,
}

#[component]
fn MenuComponent(menu_items: Signal<Vec<MenuType>>) -> Element {
    rsx! {
        ul { class: "flex flex-col md:flex-row gap-2",
            for menu in menu_items.read().iter() {
                li { class: "flex items-center gap-2 p-2 hover:bg-gray-200 rounded cursor-pointer",
                    // Render icon berdasarkan enum
                    match menu.icon {
                        MenuIcon::Transfer => rsx! {
                            Icon {
                                icon: BsTrashFill,
                                width: 20,
                                height: 20,
                                fill: "currentColor",
                            }

                        },
                        MenuIcon::Withdraw => rsx! {
                            Icon {
                                icon: BsArrowBarDown,
                                width: 20,
                                height: 20,
                                fill: "currentColor",
                            }
                        },
                    }
                    span { "{menu.name}" }
                }
            }
        }
    }
}

#[component]
pub fn Home() -> Element {
    let menu = use_signal(|| {
        vec![
            MenuType {
                name: "transfer".to_string(),
                icon: MenuIcon::Transfer,
            },
            MenuType {
                name: "tarik tunai".to_string(),
                icon: MenuIcon::Withdraw,
            },
        ]
    });

    rsx! {
        main { class: "h-screen w-screen bg-[#d9d9d9] relative",
            // logo bagian utama paling atas
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
            // bagina daftar beat coin
            div { class: "flex items-center justify-center",
                div {
                    Icon {
                        icon: FaBitcoinSign,
                        width: 40,
                        height: 40,
                        fill: "currentColor",
                    }
                    p { "pendapatn bunga " }
                    h4 {
                        "Rp :"
                        {"12121212"}
                    }
                }
                div {
                    p { "------------ crypto -------------" }
                }
            }
            // bagian menu dan chard
            div {
            }
            // bagian menu menu dan chard
            div { class: "flex flex-col md:flex-row items-center justify-center gap-4",
                div { class: "border-2 bg-[#ffffff] rounded-lg p-4 shadow-md w-[80%] md:w-[40%]",
                    p { "chard" }
                }
                // menu
                div { class: " bg-[#ffffff] rounded-lg p-4 shadow-md w-[80%] md:w-[40%]",
                    MenuComponent { menu_items: menu }
                }
            }
        }
        Navigation {}
    }
}
