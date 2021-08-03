# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.3

### Added

- Added dairy and milk business
- Allow selecting production for buildings if they have multiple
- Added animal feed business
- Indicate when production is idle
- Indicate which building or car is selected
- Allow selecting map sizes when creating a new map
- Added initial menu
- Allow for cloning car instructions onto other cars

### Changed

- Storages now look more different
- Decrease production rate and speed and increase storage sizes
- Group items in export station

### Fixed

- Fixed issue where pathfinding crashed games when deleting things

## 0.1.2

### Added

- Allow deactivating and activating cars
- Allow for differently looking cars depending on resource
- Allow editing names of buildings and cars
- Add several agriculture processes

### Changed

- Cars automatically select correct resource type for loading and unloading
- Grouped items in construction menu for better overview
- Resources are loaded from asset files

### Fixed

- Cars are rendered as sprites, leading to a smoother movement and no blinking effect

## 0.1.1

### Added

- Centered map when starting game
- Zoom towards mouse cursor
- Allow configuring what export stations export
- Added instructions Load and Unload (without waiting) to cars

### Changed

- Improved design of tiles, updated storages to look different depending on resource
- Allow dragging streets to place them
- Refactored production buildings to be loaded from asset file

### Fixed

- Resolved cars dead-locking due to blocking each other

## 0.1.0

### Added

- Initial release with steel metal infrastructure
