<h1 align="center">asserhttp</h1>
<div align="center">
 <strong>
   Fluent http assertions
 </strong>
</div>
<br />
<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/asserhttp">
    <img src="https://img.shields.io/crates/v/asserhttp.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/asserhttp">
    <img src="https://img.shields.io/crates/d/asserhttp.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/asserhttp">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
  <!-- license -->
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/license-Apache_2-blue.svg?style=flat-square"
      alt="Apache 2" />
  </a>
  <!-- CI status -->
  <a href="https://github.com/beltram/asserhttp/actions">
    <img src="https://github.com/beltram/asserhttp/workflows/ci/badge.svg?style=flat-square"
      alt="ci" />
  </a>
  <!-- Code coverage -->
  <a href="https://coveralls.io/github/beltram/asserhttp?branch=main">
    <img src="https://coveralls.io/repos/github/beltram/asserhttp/badge.svg?branch=main" alt="coverage" />
  </a>
</div>
<br/>

A standard trait for doing fluent assertions over many http client response. Currently, supports 
[surf](https://github.com/http-rs/surf) and [isahc](https://github.com/sagebind/isahc).