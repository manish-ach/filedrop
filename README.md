# ScreenShot

<img width="1366" height="768" alt="image" src="https://github.com/user-attachments/assets/51d4c581-417f-4767-9b22-d388c4eab667" />

<details>
  <summary>Old Screenshot logs</summary>
  
  <img width="1366" height="768" alt="image" src="https://github.com/user-attachments/assets/8178f528-4b38-4cbf-9f15-6244150a3e13" />

</details>

# Development

Your new bare-bones project includes minimal organization with a single `main.rs` file and a few assets.

```
project/
├─ assets/ # Any assets that are used by the app should be placed here
├─ src/
│  ├─ main.rs # main.rs is the entry point to your application and currently contains all components for the app
├─ Cargo.toml # The Cargo.toml file defines the dependencies and feature flags for your project
```

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform web
```

