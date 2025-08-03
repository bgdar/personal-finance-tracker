# Personal Finance Tracker

apikasi yang mengelola **keuangan**

<p align="center">
  <!-- Skill Icons -->
  <a href="https://skillicons.dev">
    <img src="https://skillicons.dev/icons?i=vscode,rust,tailwind,html,css,js,ts&perline=4" />
  </a> <br />
  
  <!-- Custom Badges -->
  <img src="https://img.shields.io/badge/Dioxus-Latest-orange?logo=rust&logoColor=white" />
  <img src="https://img.shields.io/badge/Editor-VSCode-blue?logo=visualstudiocode&logoColor=white" />
  <img src="https://img.shields.io/badge/Language-Rust-brown?logo=rust&logoColor=white" />
  <img src="https://img.shields.io/badge/Framework-Dioxus-orange?logo=rust&logoColor=white" />
  <img src="https://img.shields.io/badge/Styling-TailwindCSS-06B6D4?logo=tailwindcss&logoColor=white" />
</p>

### Tech stack

1. `Dioxus` : cross platfrom framework
2. `Tailwindcss ` : styling app
   Dalam mode dev , jalanakan :
   ```bash
   npx @tailwindcss/cli -i ./input.css -o ./assets/tailwind.css --watch
   ```
3. `dioxus-free-icons` : all icons
   menggunakan 3 jenis icons :
   - "font-awesome-solid"
   - "bootstrap"
   - "ionicons"

### Running

Run the following command in the root of your project to start developing with the default platform:
To run for a different platform, use the `--platform platform` flag. E.g.

```bash
# jalankan default (web)
dx serve
# jalanakn dengan spesifik platfrom
dx serve --platform web
dx serve --platform mobile
dx serve --platform desktop
```
