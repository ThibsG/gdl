# metallic

#### Graphic Design with WebAssembly

Metallic is a high-performance Rust 2D graphic design library, which compiles to WebAssembly, allowing developers to create 2D graphics 
at near-native speed in the browser, and natively on their machines. 

Metallic allows you to create:
- Image collages
- Social media graphics
- Banners
- Adverts 

Metallic can be used in-tandem with the Photon image processing library for combined effects and further image manipulation at a lower level. Metallic intends to be a wrapper over Photon, perfect for developers who wish to incorporate high-level graphic design functionality into their web apps and native apps. 


##### Features:
- **Pure Rust** - Unlike other libraries, this library is built with 100% pure Rust, so security and safety is guaranteed. 
- **WebAssembly friendly** - For web-based graphic design, Metallic is 4-10x faster than JS, leading to faster results, and less lag. 
- **Call WASM with JS** - This library's has exposed JS functions, allowing for zero-cost abstraction and faster development.
- **Over 90 functions** - metallic provides functions for every domain of image processing. 

##### Functions

### 🧐 Repo Organisation
This repo can be thought of as a hybrid library, divided into 2 major components:
1. ***Core Rust library***: This provides universal graphic design functions, which can be used either natively or in the browser. The WASM component of this repo relies on this for all image processing functionality. 
2. ***WebAssembly helpers and starter demo***: This section of the library contains helpers for exporting pixel data to and from the browser, and communicates directly with the core Rust library. The starter demo demonstrates calling the compiled WebAssembly code using JS functions and hooks into a webpack build pipeline. 

### Live Demo
View the [official demo of WASM in action](https://silvia-odwyer.github.io/metallic).

### Documentation
Documentation can be found [here](https://silvia-odwyer.github.io/metallic/docs/metallic/index.html).

## Examples
![](https://github.com/silvia-odwyer/metallic/blob/master/docs/img_examples/streetlamp_collage.png)


## 🚴Getting Started

These instructions will get you a copy of metallic up and running on your local machine for development and testing purposes. See deployment for notes on how to deploy the project on a live system.

### Prerequisites

To use metallic, you must have Rust and Node installed. Builds of metallic are compiled using the rust nightly toolchain.

### Installing
<!-- ## Cargo Status -->
<!-- `metallic` can be installed via Cargo by declaring the following dependency in your Cargo.toml file:
```toml
[dependencies]
metallic-rs = "*"
``` -->

Clone this repo:
```sh
git clone https://github.com/silvia-odwyer/metallic
```

Run the binary, which will perform an image processing function on an image:
```sh
cd crate
cargo run --release 
```

Compare the original image with the outputted image, and you'll see the desired effect has been applied.

### Native Use
metallic contains native-only functions for opening files from the filesystem. 

When an image is opened, it is converted to a `metallicImage` type, which can then be passed into any image processing function, and the `metallicImage` value is accordingly edited.

Getting started is relatively straightforward, this code snippet is all you need to get started:
```rust
extern crate metallic;
fn main() {

}
```

See the documentation for a full list of effects which you can apply. All functions take in a `metallicImage` similar to above.

### 🚴 Get Started With WebAssembly 

##### 🔋 Batteries Included

This repo comes pre-configured with a quick-start demo, which hooks into a Webpack build pipeline, and provides all WASM-friendly functions.

***WARNING***: Running WASM code in development mode is ***significantly*** slower than in production mode (often up to 10 times),
so don't be disheartened if the JS alternatives outperform WASM. For the blazing speeds promised, make sure to build the 
project under production mode with `npm run build` and visit `dist/index.html`. 

* `npm run start` -- Serve the project locally for development at
  `http://localhost:8080`.

* `npm run build` -- Bundle the project (in production mode).

##### WebAssembly Use
To allow for universal communication between the core Rust library and WebAssembly, the functions have been generalised to allow for both native and in-browser use. 

Due to this, image data from the browser must first be converted to a metallicImage before being passed to the image processing functions. 

The metallicImage can then be converted back to JS-compatible ImageData so that it can be displayed in-browser.

See the code snippet below:
```js
function filterImage(event) {
    var canvas = document.getElementById("canvas");
    var ctx = canvas.getContext("2d");
    
    ctx.drawImage(newimg, 0, 0);
    
    // Get the image data from the image
    let imgData = ctx.getImageData(0, 0, canvas.width, canvas.height);

    // Convert the ImageData to a metallicImage (so that it can communicate with the core Rust library)
    let rust_image = module.open_image(imgData, canvas.width, canvas.height);

    // Filter the image, the metallicImage's raw pixels are modified and 
    // the metallicImage is returned
    let new_image = module.filter(rust_image, filter_name);

    // Convert the metallicImage's raw pixels to JS-compatible ImageData
    let new_pixels = module.to_image_data(new_image);
    
    // Place the pixels back on the canvas
    ctx.putImageData(new_pixels, 0, 0);
  }
```

Not all functions available in the core Rust library are available in WebAssembly (currently investigating this). Only WASM-friendly functions have been annotated with #[wasm_bindgen]. All supported WASM functions are displayed in the starter demo. 

## Modules 
metallic contains a series of modules, which include:

- `effects`: Various image effects, including adding offsets, thresholding, duotoning, solarization, etc.,
- `channels`: Functions related to increasing/decreasing the red, green, and blue channels of the image data.
- `filters`: Preset filters, which alter the rgb channels of the image. Contains over 20. 
- `conv`: Laplace, Sobel, emboss; image proc functions which require image convolution. 
-  `noise`: Noise generation of varying tints and hues. 
- `multiple`: A module for dealing with multiple images, such as watermarking images, etc.,
- `correction`: Hue rotation, adjusting saturation, lightening/darkening: all techniques available in multiple colour spaces, which lead to varying effects.

All effects and filters can be viewed below and on the official website.

## 📚 Documentation
View the official [documentation here](https://silvia-odwyer.github.io/metallic/docs/metallic/index.html). 

## Building For Production

#### Native
You can edit the binary file in the `bin` dir to create an executable or a command-line app, for example. 

Then, to build this executable for native use in release mode:

```sh
cd crate 
cargo build --release
```

#### WebAssembly
To build the example under production mode:

```sh
npm run build
```

Check the `dist` folder for the outputted static files, which can be deployed to a live server.

##  Using This Template

You can use `npm init` to clone this template:

```sh
npm init rust-webpack my-app
```

[Afterwards check out the full documentation for exploring it][template-docs].

## Contributing

metallic is always ready for new filters and functions, so if you'd like to contribute, we're always ready to accept new Pull Requests or investigate new issues. 

## Authors

* **Silvia O'Dwyer** - [GitHub Profile](https://github.com/silvia-odwyer)
* **Future You(?)** - (See Contributing above ;) 

## License

This project is licensed under the Apache 2.0 License - see the [LICENSE.md](LICENSE.md) file for details.