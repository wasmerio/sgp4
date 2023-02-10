<p align='center'>
    <h2  align='center'><span>Sgp4 for </span><span  style="color: #654ef1">WAPM</span></h2>
    <p align='center'>
    <a title="Hosted on WAPM" href="https://wapm.io/dynamite-bud/sgp4">
    <img src="https://img.shields.io/badge/WAPM-654ef1.svg?style=flat-square&logo=WebAssembly&labelColor=654ef1&logoColor=fff">
    </a>
    <a title="GitHub License button" href="https://github.com/wasmerio/sgp4/blob/main/LICENSE_APACHE.md"><img src="https://img.shields.io/github/license/wasmerio/sgp4.svg?style=flat-square"></a>
    <a title="GitHub MIT license button" href="https://github.com/wasmerio/sgp4/blob/main/LICENSE_MIT.md"><img src="https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square"></a>
  </p>
</p>

---

This repository is the port of the [SGP4](https://crates.io/crates/sgp4) library to WebAssembly.

Published package is available on [WAPM](https://wapm.io/dynamite-bud/sgp4).

## Installation

- **`WAPM`**
  ```console
  $ wapm install dynamite-bud/sgp4
  ```
- **`npm`**
  ```console
  $ wapm install dynamite-bud/sgp4 --npm
  ```
- **`pip`**
  ```console
  $ wapm install dynamite-bud/sgp4 --pip
  ```

## Usage with JavaScript

```js
const { bindings } = require("@dynamite-bud/sgp4");
const {
  Elements,
  Constants,
} = require("@dynamite-bud/sgp4/src/bindings/sgp4/sgp4.js");

const resolveResult = ({ tag, val }) => (tag === "err" ? new Error(val) : val);
const wasm = await bindings.sgp4();
let response = await fetch(
  "https://celestrak.com/NORAD/elements/gp.php?GROUP=galileo&FORMAT=json"
);
let elementsArr = (await response.json()).map((e) =>
  resolveResult(Elements.fromJson(wasm, JSON.stringify(e)))
);
for (let elements of elementsArr) {
  console.log(elements.getObjectName());
  let constants = resolveResult(Constants.fromElements(wasm, elements));
  for (let hours of [12, 24]) {
    console.log(`    t = ${hours * 60} min`);
    let prediction = resolveResult(constants.propagate(parseFloat(hours * 60)));
    console.log(`        r = ${prediction.position} km`);
    console.log(`        ṙ = ${prediction.velocity} km.s⁻¹`);
  }
}
```

For more information, please refer to the `sgp4` [documentation](https://docs.rs/sgp4/0.9.1/sgp4/index.html).
