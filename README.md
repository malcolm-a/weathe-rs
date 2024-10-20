<!-- TOC -->
<details>
  <summary>🌦️ <strong>weathe-rs</strong></summary>
  <ul>
    <li>📖 <strong><a href="#about">About</a></strong>
      <ul>
        <li><strong><a href="#purpose">Purpose</a></strong></li>
        <li><strong><a href="#demo">Demo</a></strong></li>
      </ul>
    </li>
    <li>🛠️ <strong><a href="#installation">Installation</a></strong></li>
  </ul>
</details>
<!-- TOC -->

# weathe-rs

## 📖 About

### Purpose

This is a pretty straightforward Rust project that displays current weather information about the city you enter.

### Demo

By default, you will see the weather for the city your IP address is located in. Here is how it looks:

![](captures/paris.png)

But you can also search for the city you want with the search bar at the top:

![](captures/search.png)

The `<Reset>` button resets to your default location.

The `<Quit>` button quits the app.

## 🛠 Installation

Clone this project with 

`git clone https://github.com/malcolm-a/weathe-rs.git`

You will need to obtain a free [weatherapi](https://www.weatherapi.com) API in order to run this project with your own API key.
Put this key in `src/api_key.rs` :

```rust
pub const API_KEY: &str = "YOUR_API_KEY";
```

Finally, run `cargo run` at the root of the project directory to run the project.