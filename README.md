
<div id="top"></div>


# Limit Order Book Reconciliation

## D2X Technical Interview

</div>

## About The Project
<p>
I wrote a program that takes connects to the Deribit API and subscribes to the book.BTC-PERPETUAL.100ms. <br>
The first snapshot of the limit order book and it's subsequents deltas are ordered in two binary tree maps. One holds the Price/Quanitity values of the asks in the book and the other for the bids.<br>
The Snapshot gives me the current state of the book.<br>
The subsequent changes can change the Limit Order Book in 3 different ways <br>

* New
    * Insert a new price level with the according quantity
* Delete
    * Remove a price level from the Limit Order Book
* Change 
    * Change the quantity of a price level

The best bid/ask price are printed out each second with their according quantities.
</p>

### Built With Rust Crates

* [Tungstenite](https://docs.rs/tungstenite/latest/tungstenite/)
* [Serde](https://docs.serde.rs/serde/index.html)
* [Serde_json](https://docs.serde.rs/serde_json/macro.json.html)
* [ordered-float](https://docs.rs/ordered-float/latest/ordered_float/)

## Installation

1. Install rust
  ```sh
  $ curl https://sh.rustup.rs -sSf | sh
  ```
2. Add the following line to your ~/.bash_profile  
  ```sh
  export PATH="$HOME/.cargo/bin:$PATH
  ```
3. Update rust to the latest version with
  ```sh
  rustup update
  ```
  4. Clone the repo
   ```sh
   git clone https://github.com/vanaacken/lobr.git 
   ```
   5. Navigate to the repo
   ```sh
    cd lobr
   ```
## Usage

when in the root of the project run
```sh
cargo run --release
```

## Contact

Niels van Aacken - [@nielsvanaacken](https://www.linkedin.com/in/nielsvanaacken/) - nielsfhvanaacken@gmail.com

Project Link: [https://github.com/vanaacken/lobr.git ](https://github.com/vanaacken/lobr.git )


[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/othneildrew
[product-screenshot]: images/screenshot.png








