# Changelog

## [0.13.1](https://github.com/hyperkliv/samling/compare/samling-v0.13.0...samling-v0.13.1) - 2025-01-08

### Fixed

- Remove direct dependency on deadpool-postgres and tokio-postgres

## [0.13.0](https://github.com/hyperkliv/samling/compare/samling-v0.12.0...samling-v0.13.0) - 2025-01-07

### Refactor

- [**breaking**] Move from cornucopia to clorinde

## [0.12.0](https://github.com/hyperkliv/samling/compare/v0.11.2...v0.12.0) (2025-01-05)


### ⚠ BREAKING CHANGES

* Upgrade to axum 0.8

### Bug Fixes

* Expose deadpool_postgres and cornucopia_async::GenericClient in samling API ([86bbe02](https://github.com/hyperkliv/samling/commit/86bbe02955cf425b5de70f19cf37f3f7650874b5))
* Expose tokio_postgres in public API ([7170d78](https://github.com/hyperkliv/samling/commit/7170d78722e5fa1e721f73ccfaf53b55cc2f988a))
* Move from cornucopia_async to maintained crate cornucopi_async ([818ff48](https://github.com/hyperkliv/samling/commit/818ff48867e47bf743c36ba30713d6825cd5abc0))
* Upgrade dependencies (except latest axum) ([d4a8a0a](https://github.com/hyperkliv/samling/commit/d4a8a0a8b5a90d1e98c11c1391c9cd46272d019e))
* Upgrade to axum 0.8 ([15ea940](https://github.com/hyperkliv/samling/commit/15ea94077eca8b7765da8e3a7d5fc3bee2ff9f1e))

## [0.11.2](https://github.com/hyperkliv/samling/compare/v0.11.1...v0.11.2) (2024-10-31)


### Bug Fixes

* **ui:** Translations weren't updating in some parts ([7dbe91a](https://github.com/hyperkliv/samling/commit/7dbe91a9d8b215399ce9a268622843616aaabc1f))

## [0.11.1](https://github.com/hyperkliv/samling/compare/v0.11.0...v0.11.1) (2024-10-31)


### Bug Fixes

* Column header translations weren't updated ([9d51fbc](https://github.com/hyperkliv/samling/commit/9d51fbc3434bfefb6502e0d245c893cc44e1e01c))
* **export:** Sort attribute columns by ID and not name ([9bf965d](https://github.com/hyperkliv/samling/commit/9bf965d6150a62dbb9409f75b7116079ed622fc1))

## [0.11.0](https://github.com/hyperkliv/samling/compare/v0.10.1...v0.11.0) (2024-10-30)


### Features

* Choose language for exported data ([538f229](https://github.com/hyperkliv/samling/commit/538f2295226180f356b91c086eb5e57cd24b0d4b))

## [0.10.1](https://github.com/hyperkliv/samling/compare/v0.10.0...v0.10.1) (2024-10-30)


### Bug Fixes

* DE locale files didn't exist ([95e874c](https://github.com/hyperkliv/samling/commit/95e874c6f17e0428164fcfbf0bb8afc9c8008e1e))

## [0.10.0](https://github.com/hyperkliv/samling/compare/v0.9.2...v0.10.0) (2024-10-30)


### Features

* Support german ([c1f36b1](https://github.com/hyperkliv/samling/commit/c1f36b13f5abdfd7995eae8ec24c7224ae394da4))

## [0.9.2](https://github.com/hyperkliv/samling/compare/v0.9.1...v0.9.2) (2024-06-19)


### Bug Fixes

* New release ([b7b2c9b](https://github.com/hyperkliv/samling/commit/b7b2c9bf2d9297935bdea54e68560cf37fc957a0))
* New release ([175878d](https://github.com/hyperkliv/samling/commit/175878d7a890e750444b5180a1094cc6dd2108c4))

## [0.9.1](https://github.com/hyperkliv/samling/compare/v0.9.0...v0.9.1) (2024-06-19)


### Bug Fixes

* Update sentry and rust_xlsxwriter ([1782d19](https://github.com/hyperkliv/samling/commit/1782d19d4eaa9f67e5052c67990c9974835b3db2))

## [0.9.0](https://github.com/hyperkliv/samling/compare/v0.8.0...v0.9.0) (2024-06-19)


### Bug Fixes

* Update dependencies ([791c44a](https://github.com/hyperkliv/samling/commit/791c44a8908422a6754ea42a50360691d2fa133f))

## [0.8.0](https://github.com/hyperkliv/samling/compare/samling-v0.7.1...samling-v0.8.0) (2024-05-16)


### Bug Fixes

* Upgrade reportedly incompatible dependencies ([ac94159](https://github.com/hyperkliv/samling/commit/ac94159c10eb6e115c7b68235e4b60656f6842eb))

## [0.7.1](https://github.com/hyperkliv/samling/compare/v0.7.0...v0.7.1) (2024-05-16)


### Bug Fixes

* Update field order in exports ([d32fa97](https://github.com/hyperkliv/samling/commit/d32fa97f54dee7322b2a8547e83b7affd2fafaf5))

## [0.7.0](https://github.com/hyperkliv/samling/compare/v0.6.0...v0.7.0) (2024-01-03)


### ⚠ BREAKING CHANGES

* Upgrade dependencies

### Bug Fixes

* Bearer value not sent correctly to Cloudflare ([7a18ee5](https://github.com/hyperkliv/samling/commit/7a18ee5f5d88635d715da6ebdeec23732c68f18c))


### Miscellaneous Chores

* Upgrade dependencies ([7ee84b7](https://github.com/hyperkliv/samling/commit/7ee84b7b7c5a8497008138ff64288729a8ae40fe))

## [0.6.0](https://github.com/hyperkliv/samling/compare/v0.5.5...v0.6.0) (2023-12-06)


### ⚠ BREAKING CHANGES

* Update to http 1.0, hyper 1.0, axum 0.7 and more
* Update dependencies

### Bug Fixes

* Allow partial comparison of different ref variants ([10a006d](https://github.com/hyperkliv/samling/commit/10a006df3dd9d6138b562ff5ef0109d36d730952))
* Choose latest uploaded image for same position ([04b5cbd](https://github.com/hyperkliv/samling/commit/04b5cbd31260471663f25d56c8dd3d0d39f9f208))
* Incorrect clone implementation for Id&lt;T&gt; ([2ee0a01](https://github.com/hyperkliv/samling/commit/2ee0a014aa31d9ae5caf8b4086a4442dbcc88dac))


### Miscellaneous Chores

* Update dependencies ([ba6ca49](https://github.com/hyperkliv/samling/commit/ba6ca498e5845a9e8de0cef13402092615d8f832))
* Update to http 1.0, hyper 1.0, axum 0.7 and more ([c77af3e](https://github.com/hyperkliv/samling/commit/c77af3ef0fa5a333dea5d4eb45b2f5b5aecbd7a9))

## [0.5.5](https://github.com/hyperkliv/samling/compare/v0.5.4...v0.5.5) (2023-05-23)


### Bug Fixes

* Move to maintained dotenv crate alternative ([fba9cb8](https://github.com/hyperkliv/samling/commit/fba9cb8610469695006c22175c604a51bbfc26a3))

## [0.5.4](https://github.com/hyperkliv/samling/compare/v0.5.3...v0.5.4) (2023-05-23)


### Bug Fixes

* Update dependencies ([50fdcf1](https://github.com/hyperkliv/samling/commit/50fdcf19ceaf9c65403a47459a555ca871bc2018))
* Use kubernetes native LB with traefik ([ca28120](https://github.com/hyperkliv/samling/commit/ca28120b43fd52987db4aec88e48672a9804b18f))

## [0.5.3](https://github.com/hyperkliv/samling/compare/v0.5.2...v0.5.3) (2023-05-10)


### Bug Fixes

* **admin:** Color enabled checkbox wasn't applied properly ([fe0b4b9](https://github.com/hyperkliv/samling/commit/fe0b4b9e4ee797d42eb8dd298f169edebd44002b))
* Remove unused collection filter from pricelist summary endpoint ([4a94ec3](https://github.com/hyperkliv/samling/commit/4a94ec3ae5d8ee7ea5e66fec34dd1c592d1009c4))

## [0.5.2](https://github.com/hyperkliv/samling/compare/v0.5.1...v0.5.2) (2023-05-10)


### Bug Fixes

* Don't remove style attribute associations unnecessarily ([ff01211](https://github.com/hyperkliv/samling/commit/ff01211c46272d1fa92e9176edd108afc32422f4))

## [0.5.1](https://github.com/hyperkliv/samling/compare/v0.5.0...v0.5.1) (2023-05-09)


### Bug Fixes

* Resolve extremely slow response compression ([6b85c96](https://github.com/hyperkliv/samling/commit/6b85c96e973b6ab99cf3356336740b7887218f19))
* Update dependencies ([b3014bf](https://github.com/hyperkliv/samling/commit/b3014bf71e0918fe1df48032225742da9acd08ca))

## [0.5.0](https://github.com/hyperkliv/samling/compare/v0.4.1...v0.5.0) (2023-05-08)


### Features

* Ability to filter by attributes in collection admin ([2717c1a](https://github.com/hyperkliv/samling/commit/2717c1a0972613cdfefbbd997bfdcd61f009eaf6))


### Bug Fixes

* **admin:** Attribute filtering of different types are now AND:ed together ([4832344](https://github.com/hyperkliv/samling/commit/48323441357c928b23879539943e3b50a6e75a04))
* Update dependencies ([dc2d42e](https://github.com/hyperkliv/samling/commit/dc2d42e18595a0063bd254a008753d096f8b7a5a))

## [0.4.1](https://github.com/hyperkliv/samling/compare/v0.4.0...v0.4.1) (2023-02-28)


### Bug Fixes

* Don't allow collection.organization_id to be null ([602cc56](https://github.com/hyperkliv/samling/commit/602cc563fb3818a74aa8c6d426d8cc568e0bf966))
* Update dependencies ([dcbf68a](https://github.com/hyperkliv/samling/commit/dcbf68a2a0f4558642b31abc5811cf2124b051da))

## [0.4.0](https://github.com/hyperkliv/samling/compare/v0.3.1...v0.4.0) (2023-01-28)


### Features

* **admin:** Item filter choices endpoint ([b385709](https://github.com/hyperkliv/samling/commit/b38570909a5a0a88034d0193b2b922be0318c9f6))


### Bug Fixes

* Update dependencies ([448b165](https://github.com/hyperkliv/samling/commit/448b165ce651478c1b94899a7007ae72b887d0f9))

## [0.3.1](https://github.com/hyperkliv/samling/compare/v0.3.0...v0.3.1) (2023-01-17)


### Bug Fixes

* New release ([bc2069c](https://github.com/hyperkliv/samling/commit/bc2069c436e00b730f405b345d6c4811a9e056e9))

## [0.3.0](https://github.com/hyperkliv/samling/compare/v0.2.2...v0.3.0) (2023-01-16)


### Features

* **cli:** Add `version` command ([2d9feca](https://github.com/hyperkliv/samling/commit/2d9fecaa34ac0d5058020099e516fb054c959e76))


### Bug Fixes

* **export:** Replace "Yes (0) No (0)" with "No" for New Color field ([4189819](https://github.com/hyperkliv/samling/commit/4189819794082eb25874e78729a05a5c48d2eb7c))
* Prices weren't showing on lower resolutions in collection table ([3bebd4a](https://github.com/hyperkliv/samling/commit/3bebd4a43eda09dc5364ce2ddbeaae01ffd09980))

## [0.2.2](https://github.com/hyperkliv/samling/compare/v0.2.1...v0.2.2) (2023-01-10)


### Bug Fixes

* Add profile image to user admin table ([58fe93d](https://github.com/hyperkliv/samling/commit/58fe93d9b34e2ec6517c49bb1aa62b33ba350f70))
* **admin:** User wasn't added to organization on create ([214f2b0](https://github.com/hyperkliv/samling/commit/214f2b0f15e44a6c27bf6cc87becb171d6ec4253))
* **admin:** Users without groups weren't visible ([ae0611b](https://github.com/hyperkliv/samling/commit/ae0611b9c4c14786f27eec108059a9d7d9d16de4))

## [0.2.1](https://github.com/hyperkliv/samling/compare/v0.2.0...v0.2.1) (2023-01-10)


### Bug Fixes

* New release ([4b603ae](https://github.com/hyperkliv/samling/commit/4b603ae91edbc6f6614bb2f3745c70f5431ed7ef))

## [0.2.0](https://github.com/hyperkliv/samling/compare/v0.1.5...v0.2.0) (2023-01-06)


### Features

* New feature to enable Mimalloc allocator ([026cf77](https://github.com/hyperkliv/samling/commit/026cf7729febb649d1a58c1e3f5af29081187d58))


### Bug Fixes

* Only enable used tokio features ([a7d1025](https://github.com/hyperkliv/samling/commit/a7d1025481cafc59bb34ab14d7941c10db1524a3))
* Use tokio 1.24 ([9e50773](https://github.com/hyperkliv/samling/commit/9e50773d99db425d1ad38af87cea6621938815b7))

## [0.1.5](https://github.com/hyperkliv/samling/compare/v0.1.4...v0.1.5) (2023-01-05)


### Bug Fixes

* New release ([36e4368](https://github.com/hyperkliv/samling/commit/36e4368c529d455f18e37a92f708e59c64a8a8cd))

## [0.1.4](https://github.com/hyperkliv/samling/compare/v0.1.3...v0.1.4) (2023-01-05)


### Bug Fixes

* New release ([62af56a](https://github.com/hyperkliv/samling/commit/62af56ad5e73baf3f537a3d4f351d2f7e93c883a))

## [0.1.3](https://github.com/hyperkliv/samling/compare/v0.1.2...v0.1.3) (2023-01-05)


### Bug Fixes

* New release ([ac12b40](https://github.com/hyperkliv/samling/commit/ac12b404c269c59398561348af8224e4c06cd124))

## [0.1.2](https://github.com/hyperkliv/samling/compare/v0.1.1...v0.1.2) (2023-01-05)


### Bug Fixes

* New release ([f41f9ff](https://github.com/hyperkliv/samling/commit/f41f9ff61873173972c255b34037b798e354b9c5))

## [0.1.1](https://github.com/hyperkliv/samling/compare/v0.1.0...v0.1.1) (2023-01-04)


### Bug Fixes

* New release ([1bc499e](https://github.com/hyperkliv/samling/commit/1bc499eac6bb257d01f84ca6649e0ca93d08b653))

## 0.1.0 (2023-01-04)


### Features

* Initial commit ([ead04e6](https://github.com/hyperkliv/samling/commit/ead04e649339a0f8f3905dcadaf1eee9528c0904))


### Bug Fixes

* Add Cargo.toml properties required for publishing ([ecfe4c2](https://github.com/hyperkliv/samling/commit/ecfe4c2407ef904277c0f56d38e680aee19068b3))
