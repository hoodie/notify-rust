# Changelog

### [v4.5.7](https://github.com/hoodie/notify-rust/compare/v4.5.6...v4.5.7) (2022-03-20)


#### Fixes

* **deps:** update rust crate mac-notification-sys to 0.5.0
 1f3a9f5


### [v4.5.6](https://github.com/hoodie/notify-rust/compare/v4.5.5...v4.5.6) (2022-02-04)


#### Fixes

* update crates zbus to v2, zvariant to v3, zvariant_derive to v3
 14bca58

* **deps:** update rust crate zbus to v2
 04901a8

* **deps:** update rust crate image to 0.24
 c7fa276


### [v4.5.5](https://github.com/hoodie/notify-rust/compare/v4.5.4...v4.5.5) (2021-11-04)


#### Fixes

* **deps:** update rust crate winrt-notification to 0.5
 6620110


### [v4.5.4](https://github.com/hoodie/notify-rust/compare/v4.5.3...v4.5.4) (2021-10-08)


#### Fixes

* update winrt-notification to 0.4
 c94e111


### [v4.5.3](https://github.com/hoodie/notify-rust/compare/v4.5.2...v4.5.3) (2021-09-16)


#### Fixes

* clear up documentation and flip env switch for dual stack
 3a3b175


### [v4.5.2](https://github.com/hoodie/notify-rust/compare/v4.5.1...v4.5.2) (2021-05-14)


#### Fixes

* clear up documentation of action handling
 ad35d34


### [v4.5.1](https://github.com/hoodie/notify-rust/compare/v4.5.0...v4.5.1) (2021-05-13)


#### Fixes

* export of ActionResponse
 3bf2f37


## [v4.5.0](https://github.com/hoodie/notify-rust/compare/v4.4.1...v4.5.0) (2021-05-05)


### Features

* **NotificationHandle:** add close reason handling
 01f9980


### [v4.4.1](https://github.com/hoodie/notify-rust/compare/v4.4.0...v4.4.1) (2021-05-01)


#### Fixes

* notify with __closed when notification is closed in zbus implementation
 3a9c206


## [v4.4.0](https://github.com/hoodie/notify-rust/compare/v4.3.0...v4.4.0) (2021-04-25)


### Features

* add schedule_raw() for f64 timestamps
 b8f811b

* add schedule method to Notification
 30f1741

* create macos schedule_notification method
 72bda94


## [v4.3.0](https://github.com/hoodie/notify-rust/compare/v4.2.2...v4.3.0) (2021-02-27)


### Features

* Convert DynamicImage::ImageRgba8
 87e92b5

* Implement TryFrom trait for RgbImage and RgbaImage
 69c2b1e

* Implement converting image with alpha
 d25ab47

* make zbus backend the default
 582b87e


### [v4.2.2](https://github.com/hoodie/notify-rust/compare/v4.2.1...v4.2.2) (2021-01-08)


#### Fixes

* remove another stray println
 bd6ab59


### [v4.2.1](https://github.com/hoodie/notify-rust/compare/v4.2.0...v4.2.1) (2021-01-08)


#### Fixes

* **deps:** update zbus
 684d031

* **deps:** update rust crate dbus to 0.9
 96f84f4


## [v4.2.0](https://github.com/hoodie/notify-rust/compare/v4.1.1...v4.2.0) (2021-01-08)


### Features

* make custom hints unique
 f6ec445


### Fixes

* remove stray dbug!()
 b67c1d5


### [v4.1.1](https://github.com/hoodie/notify-rust/compare/v4.1.0...v4.1.1) (2021-01-07)


#### Fixes

* remove stray println
 15b3ecd


## [v4.1.0](https://github.com/hoodie/notify-rust/compare/v4.0.0...v4.1.0) (2021-01-06)


### Features

* add zbus version
 58d38ba


### Fixes

* **deps:** update rust crate image to 0.23
 1dd236d


## [v4.0.0](https://github.com/hoodie/notify-rust/compare/v3.6.3...v4.0.0) (2020-06-06)

### ⚠ BREAKING CHANGE

* remove `From<&str>`
* restructure modules and exports


### Features

* **windows:** additions to the API (#69)
 1d9cb0e

* make notification non-exhaustive
 0304274

* make error non-exhaustive
 26f96e4

* drop redundant name prefixes
 faf3123

* restructure modules and exports
 45be84c

* .image() no longer silently fails
 8b215bd


### Fixes

* reexport NotificationHandle
 00edbc9


### [v3.6.3](https://github.com/hoodie/notify-rust/compare/v3.6.2...v3.6.3) (2019-11-02)


#### Fixes

* **deps:** update rust crate lazy_static to 1.4
 687e34d

* build again after merge conflict
 bcfc8c8

* test-build without `--features image` 🙄
 3eead0b

* test-build with `--features image`
 60e963d

* test-build with `--features image`
 92217a2


### [v3.6.2](https://github.com/hoodie/notify-rust/compare/v3.6.1...v3.6.2) (2019-08-11)


#### Fixes

* test-build without `--features image` 🙄
 0524a5f


### [v3.6.1](https://github.com/hoodie/notify-rust/compare/v3.6.0...v3.6.1) (2019-08-11)


#### Fixes

* test-build with `--features image`
 8ee6998


## [v3.6.0](https://github.com/hoodie/notify-rust/compare/v3.5.0...v3.6.0) (2019-05-06)


## [v3.5.0](https://github.com/hoodie/notify-rust/compare/v3.4.3...v3.5.0) (2018-10-21)


### [v3.4.3](https://github.com/hoodie/notify-rust/compare/v3.4.0...v3.4.3) (2018-10-13)


## [v3.4.0](https://github.com/hoodie/notify-rust/compare/v3.2.1...v3.4.0) (2017-05-21)


### [v3.2.1](https://github.com/hoodie/notify-rust/compare/v3.1.1...v3.2.1) (2016-09-07)


### [v3.1.1](https://github.com/hoodie/notify-rust/compare/v3.1.0...v3.1.1) (2016-03-03)


## [v3.1.0](https://github.com/hoodie/notify-rust/compare/v3.0.4...v3.1.0) (2016-03-01)


### [v3.0.4](https://github.com/hoodie/notify-rust/compare/v3.0.3...v3.0.4) (2016-02-15)


### [v3.0.3](https://github.com/hoodie/notify-rust/compare/v3.0.2...v3.0.3) (2016-02-15)


### [v3.0.2](https://github.com/hoodie/notify-rust/compare/v3.0.1...v3.0.2) (2016-02-15)


### [v3.0.1](https://github.com/hoodie/notify-rust/compare/v3.0.0...v3.0.1) (2015-10-23)


## [v3.0.0](https://github.com/hoodie/notify-rust/compare/v2.1.0...v3.0.0) (2015-10-01)


## [v2.1.0](https://github.com/hoodie/notify-rust/compare/v2.0.0...v2.1.0) (2015-09-27)


## [v2.0.0](https://github.com/hoodie/notify-rust/compare/v1.1.0...v2.0.0) (2015-08-04)


## [v1.1.0](https://github.com/hoodie/notify-rust/compare/v1.0.1...v1.1.0) (2015-08-03)


### [v1.0.1](https://github.com/hoodie/notify-rust/compare/v1.0.0...v1.0.1) (2015-07-19)


## [v1.0.0](https://github.com/hoodie/notify-rust/compare/v0.9.0...v1.0.0) (2015-07-12)


## [v0.9.0](https://github.com/hoodie/notify-rust/compare/v0.8.0...v0.9.0) (2015-06-30)


## [v0.8.0](https://github.com/hoodie/notify-rust/compare/v0.0.8...v0.8.0) (2015-06-19)


### [v0.0.8](https://github.com/hoodie/notify-rust/compare/v0.0.7...v0.0.8) (2015-06-19)


### [v0.0.7](https://github.com/hoodie/notify-rust/compare/v0.0.6...v0.0.7) (2015-06-13)


### [v0.0.6](https://github.com/hoodie/notify-rust/compare/v0.0.4...v0.0.6) (2015-06-08)


### [v0.0.4](https://github.com/hoodie/notify-rust/compare/v0.0.3...v0.0.4) (2015-05-30)


### [v0.0.3](https://github.com/hoodie/notify-rust/compare/v0.0.2...v0.0.3) (2015-05-24)


### v0.0.2 (2015-05-22)

