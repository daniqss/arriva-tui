# Arriva Terminal User Interface
Simple Arriva Galicia terminal client written in Rust with the [Ratatui TUI library](https://ratatui.rs/). It fetches the Arriva Galicia APIs and shows the bus stops available to select today. Once selected, it shows the different expeditions available for the two selected stops, with the departure and arrival schedule and the travel cost.

![Searching for stops](./image-1.png)

## Run
Rust and Cargo are required to run the project. To run the project, execute the following command in the project root directory:

```bash
cargo run
# or if you want the release build
cargo run --release
```

## TODO
- [ ] Stateful Expeditions UI
- [ ] Show the selected expedition details
- [ ] Separate the different UI elements in components