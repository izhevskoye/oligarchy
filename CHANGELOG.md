# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.6

### Added

- Added import stations
- Added storage management to balance resources between storage units
- Added sugar business
- Added bakery business
- Added flour processing business
- Added beer and wine making business
- Added coal ore as a prerequisite of coal
- Show statistics in UI
- Allow pausing of the game
- Generate a street when launching a game
- Added forrest and water tiles when launching a game

### Changed

- Sorted save games by name
- Highlight connected delivery stations when hovering over depots
- Added confirmation dialog to overwrite of save files

### Fixed

- Game now properly resets selected tools when exiting
- When an export station is deleted, its statistics continue to count towards the goals
- Prevent delivery station to be used while still under construction
- Prevent multiple 'go to' instructions stopping a car indefinitely

## 0.1.5

### Added

- Automatic depots which supply resources between delivery stations
- Buildings which are placed are slowly build over time, not immediately
- Proper menu
- Confirmation dialog when deleting files
- Vodka production

### Changed

- Buildings can have multiple productions active and will randomly produce a possible activated production each tick
- Exports stations export 10 items per tick instead of one
- Adjusted prices and production rates for different buildings
- Animal based buildings produce manure which can be used as fertilizer for fields
- Phosphor fertilizer production
- Allow corn to be implicitly also used as animal feed
- Allow zooming when a construction option is selected

## 0.1.4

### Added

- Add money
- Buildings cost maintenance
- Added goals for game
- Cast steel into billets, slabs and blooms
- Allow seeing production rates in UI
- Allow multiple save files

### Changed

- Move cars smoothly rather than jumping by tiles

### Fixed

- Do not allow cars to drive onto buildings (and drive off if they happen to be on one)
- Starting game clears out idle indicators properly

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
