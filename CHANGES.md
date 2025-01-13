# Changelog

## Unreleased

### Added

+ core: Add convenience methods for setting the active `StackPage` in `FactoryVecDeque` and `FactoryHashMap`
+ core: Impl `Binding` for `gtk::CheckButton`
+ core: Added a method to change column headers in `RelmColumn`. Useful for translations

### Added

+ core: Add method `FactoryVecDeque::extend` to append multiple components efficiently.

### Fixed

+ example "tracker": Display identical background when initial icons are in fact identical.

## 0.9.1 - 2024-10-11

### Added

+ core: Add gnome_47 feature flag for GNOME 47

### Changed

+ core: Add more async factory types to prelude
+ core: Add `notify_filter_changed` to typed views to allow dynamic filters
+ components: Hide recent button from `OpenButton` component when there is no entries

### Fixed

+ core: Target gtk/gnome_46 with the gnome_46 feature flag
+ components: Replace deprecated `from_pixbuf()` usage
+ components: Don't panic in `get_active_elem()` when calling on `SimpleComboBox` with empty variants
+ core: Ignore sending error if async component was dropped quickly
+ macros: Destructure fields of returned widgets
+ examples: Add example that shows embedded logo

## 0.9.0 - 2024-7-12

### Added

+ core: Add `set_align` and `set_expand` to `RelmWidgetExt` to set horizontal and vertical properties at once
+ core: Add `allow_multiple_instances()` to `RelmApp` to allow multiple concurrent instances of the same application
+ core: Add extension traits for `adw::PreferencesPage`
+ examples: drop sub-components instead of hiding
+ examples: Add example for using multiple windows
+ core: Add functions to set priority of CSS stylesheets
+ core: Rewrite `Alert` component from scratch to work with both GTK4 and Adwaita
+ css: Add Adwaita style classes and colors
+ core: Migrate codebase over to using `relm4-css`
+ components: Increase flexibility of `Alert` component
+ components: Make `Alert` component match styling of Adwaita's `MessageDialog` better

### Changed

+ core: Simplified internal code for runtime creation
+ all: update dependencies

### Fixed

+ core: Don't require `Clone` and `Debug` for the generic action name parameter in `RelmAction`
+ examples: show the dialog before closing in "components" example

## 0.8.1 - 2024-3-13

### Fixed

+ docs: Fix builds on docs.rs (by upgrading relm4-icons)
+ components: Don't use libadwaita by default

## 0.8.0 - 2024-3-13

### Changed

+ core: Switch to regular async traits in favor of the async-trait crate
+ all: Updated all dependencies to their latest version
+ all: Increase MSRV to 1.75 to match the dependencies

## 0.7.1 - 2024-3-13

### Fixed

+ docs: Fix builds on docs.rs (by upgrading relm4-icons)

## 0.7.0 - 2024-3-13

### Added

+ core: Add `Toaster` as abstraction over `adw::ToastOverlay` for usage in the model of a component
+ core: Add `TypedGridView` as idiomatic wrapper over `gtk::GridView`

### Changed

+ core: Remove unmaintained actions code
+ core: Move libadwaita examples in regular examples folder
+ core: Move `drawing` module to `abstractions`

## 0.7.0-rc.1 - 2024-2-13

### Added

+ core: Implement more traits for libadwaita 1.4 widgets
+ core: Add `size()` method to `DrawHandler` for easier access of width and height
+ core: Add method to set `RelmAction` enabled or disabled
+ core: Add method to get the inner `gio::SimpleAction` used by `RelmAction`
+ core: Add resizeable/expandable column functionality to `RelmColumn` and `LabelColum`
+ core: Pass the `Root` as owned parameter (without reference) to `Component::init`
+ core: Add `visible_on_activate()` to `RelmApp` to prevent the app window from being visible immediately
+ core: Make `into_stream()` method on `Receiver` public
+ examples: Add libadwaita `Toast` example

### Changed

+ core: Make `set_global_css` and `set_global_css_from_file` methods of `RelmApp` to prevent calling them before initializing GTK 
+ core: Always pass the `Root` as owned parameter (without reference) (affects `Component::init`, `Factory::init_widgets`, `AsyncComponent::init_loading_widgets`, `AsyncFactory::init_widgets` and `AsyncFactory::init_loading_widgets`)
+ core: Move internal initialization to `gtk::Application::startup` signal handler
+ core: Remove deprecated RelmApp methods
+ components: Increase flexibility of `Alert` component
+ core: Bump version of libpanel dependency to 0.3
+ all: Increase MSRV to 1.74 to match the dependencies

### Fixed

+ core: Report the correct dimensions in DrawingHandler when a scaling factor is set
+ core: Setting the visibility of the main window isn't overridden by `RelmApp` anymore
+ core: Fix timing of `transient_for()` when called after app init

## 0.7.0-beta.2 - 2023-10-14

### Added

+ core: Add builder for initializing async factories
+ core: Add `launch_default()` method to factory builders for launching with default parent widget
+ core: Implement `RelmContainerExt` for libadwaita 1.4's `NavigationView`
+ core: Implement `RelmSetChildExt` for libadwaita 1.4's `NavigationPage`

### Changed

+ core: Return `Result` from `FactorySender#output` method (so that errors are not silently unwrapped)
+ core: Move parameters from `builder()` to `launch()` in factories for more consistency

### Fixed

+ core: Properly re-export factory builder types and document them

## 0.7.0-beta.1 - 2023-9-23

### Added

+ core: Add `AsyncComponentStream` to supports streams for async components as well
+ core: Builder pattern for initializing factories similar to regular components
+ components: Add `SimpleComboRow` helper for libadwaita's `ComboRow`, analogous to `SimpleComboBox`
+ components: Add an example for `SimpleComboRow`
+ components: Add an example how to use `SimpleComboBox`

### Changed

+ core: Add `Init` type to the `WidgetTemplate` trait to allow passing data to `init()`
+ core: Just require `AsRef<gtk::Window>` in `RelmApp::run`
+ core: Add `AsRef<Root>` as requirement to the `WidgetTemplate` trait
+ core: Removed `forward_to_parent` from `FactoryComponent`

### Fixed

+ macros: Improve spans in code generation to prevent false positive lints
+ macros: Fix track attribute for template children
+ macros: Allow trailing commas in the last match arm

## 0.7.0-alpha.1 - 2023-8-28

### Added

+ core: Add gnome_45 feature flag for GNOME 45

### Changed

+ core: Bump version of gtk4 dependency to 0.7
+ core: Bump version of libadwaita dependency to 0.5

## 0.6.2 - 2023-8-27

### Fixed

+ macros: Add support for accessing nested template children

## 0.6.1 - 2023-8-9

### Added

+ core: Add `TypedColumnView` as a typed wrapper for `gtk::ColumnView`
+ core: Add `set_margin_vertical` and `set_margin_horizontal` to RelmWidgetExt

### Fixed

+ core: Don't panic when dropping components from asynchronous contexts
+ core: Fix an issue with using `connect_open` on `gtk::Application`
+ core: Use GNOME 42 as baseline feature to help with Ubuntu 22.04
+ core: Fix compiler error when not using the "macros" feature
+ macros: Allow trailing commas in view!
+ macros: Allow multiple instances of the same template children
+ components: Disable default features of relm4
+ examples: Fix libadwaita tab examples

### Changed

+ examples: Add a separator to the libadwaita leaflet example.

## 0.6.0 - 2023-5-31

### Added

+ core: Implemented `RelmRemoveExt` for `adw::ExpanderRow`.
+ core: Implemented `ContainerChild` for `adw::ExpanderRow`.
+ core: Add `TypedListView` as idiomatic wrapper over `gtk::ListView`

### Fixed

+ macros: Improve error messages for non-identifier parameter patterns

## 0.6.0-beta.1 - 2023-4-19

### Added

+ core: Introduce setting and action safeties
+ core: Implement `RelmSetChildExt` for `gtk::AspectFrame`
+ core: Add `FactoryHashMap` as alternative to `FactoryVecDeque`
+ core: Add gnome_44 feature flag for GNOME 44
+ core: Documentation and better support for data bindings
+ core: Add `set_tooltip` method to `RelmWidgetExt`
+ core: Add `main_adw_application` method to retrieve the `adw::Application` when the libadwaita feature is enabled
+ macros: Add `skip_init` option for watch and track attributes to skip their initialization
+ examples: Introduce setting and action safeties
+ examples: Example for using relm4-icons

### Changed

+ core: Replace `FactoryVecDeque`'s associated function `from_vec` with `from_iter`
+ core: Added `Index` type to the `FactoryComponent` trait
+ core: Rename factory component traits `output_to_parent_input` method to `forward_to_parent`
+ core: Improved `RelmActionGroup` API
+ all: Increase MSRV to 1.65 to match the dependencies

### Fixed

+ all: Fix doc links
+ core: Improve worker docs

### Removed

+ core: Remove the deprecated `send!` macro

## 0.6.0-alpha.2 - 2023-3-13

### Added

+ core: Add data bindings
+ core: Implement `FactoryView` for `adw::Leaflet`
+ components: WebImage as component for easily loading images from the web

### Fixed

+ macros: Support template children of templates that are root widgets
+ macros: Fix order of assignment and connect statements

## 0.6.0-alpha.1 - 2023-2-29

### Added

+ core: Add `RelmApp` builder methods `with_args` and `with_broker`
+ core: Add `MessageBroker` support for `AsyncComponent`

### Changed

+ core: Change `MessageBroker` generic type parameter to message type
+ core: Rename `RelmApp` method `with_app` to `from_app`
+ core: Deprecate `RelmApp` methods `run_with_args` and `run_async_with_args`

## 0.5.0 - 2023-2-17

### Added

+ core: Implement `RelmSetChildExt` for `gtk::Expander`
+ macros: Support submenus in menu! macro
+ macros: Support widget templates as root widgets of components and factories
+ macros: Implement `Clone` for widget templates

### Changed

+ all: Use docs.rs to host the documentation. The documentation on the website will be deprecated.

### Fixed

+ core: Call shutdown on components even on application shutdown
+ core: Clearing and dropping a factory properly calls the shutdown method of its elements
+ components: Fix doc links to examples on GitHub
+ macros: Fix panic on incorrect root type syntax
+ macros: Fix incorrect type generation for generics
+ macros: Allow mutably root widgets

## 0.5.0-rc.2 - 2023-2-5

### Changed

+ core: Increase MSRV to 1.64 to match the gtk4 crate
+ all: Move examples into the corresponding crates

### Added

+ core: Add `broadcast` to `FactoryVecDeque`

### Fixed

+ core: Don't crash when the application is started multiple times
+ core: Support tokio also on local futures
+ core: Prevent leaking `CommandSenderInner` struct
+ core: Improve error message when sending input messages to dropped components
+ core: Fix scaping of examples on docs.rs
+ core: Fix crash caused by UID overflow with very large or frequently changing factories
+ macros: Fix clippy warning triggered by the view macro in some edge cases
+ macros: Import `relm4::ComponentSender` isn’t longer required

## 0.5.0-rc.1 - 2022-12-21

### Added

+ core: Add `try_read` and `try_write` methods to `SharedState`
+ core: Allow initializing `FactoryVecDeque` from a `Vec` and make it cloneable
+ core: Support factories with `adw::PreferencePage`

### Changed

+ core: Pass `&self` to the `Position::position()` function for positioning widgets
+ core: Take `&str` instead of `&[u8]` in `set_global_css()`
+ macros: Allow expressions for names of menu entries

### Fixed

+ core: Initialize GTK when calling `RelmApp::new()`

## 0.5.0-beta.5 - 2022-11-26

### Added

+ core: Add asynchronous components including macro support
+ core: Add asynchronous factories including macro support
+ core: Temporary widget initialization for async components and factories
+ core: Add `LoadingWidgets` to help with temporary loading widgets in async factories and components
+ core: Add `Reducer` as message based alternative to `SharedState`
+ core: Synchronous API for commands
+ core: Remove async-broadcast dependency
+ core: Runtimes of `Component`s and `AsyncComponents` can now be detached for a static lifetime
+ core: Add `ComponentStream` as alternative to `Controller` that implements `Stream` for async message handling
+ core: Add `gnome_42` and `gnome_43` feature flags
+ core: Implement `RelmContainerExt` for `adw::Squeezer`
+ core: Implement `RelmSetChildExt` for `gtk::WindowHandle`
+ macros: Auto-generate the name of the `Widgets` type if possible

### Changed

+ core: Rename `FactoryComponentSender` to `FactorySender` and `AsyncFactoryComponentSender` to `AsyncFactorySender`
+ core: The sender API now supports proper error handling
+ core: Pass `Root` during `update` and `update_cmd` for `Component` and `AsyncComponent`
+ core: Rename `OnDestroy` to `RelmObjectExt`
+ core: Remove `EmptyRoot` in favor of the unit type
+ macros: Allow using methods calls as widget initializers in the `view` macro
+ macros: Explicitly using `visibility` as attribute name is no longer supported

### Fixed

+ all: Fix doc builds on docs.rs and add a CI job to prevent future doc failures
+ core: Fix various bugs related to component shutdown
+ core: `shutdown` on `Component` now works as expected
+ core: `shutdown` on `FactoryComponent` now works as expected
+ core: `transient_for` on `ComponentBuilder` now works properly when called after the application has been initialized
+ macros: Mark template children of public widget templates as public as well
+ macros: Only get template children in scope when they are actually used
+ macros: Fix type parsing after arrow operator in widget assignments

## 0.5.0-beta.4 - 2022-10-24

### Added

+ core: Added `dox` feature to be able to build the docs without the dependencies
+ core: Added widget templates
+ core: Allow changing the priority of event loops of components
+ core: Impl `ContainerChild` and `RelmSetChildExt` for `adw::ToastOverlay`
+ components: Added `dox` feature to be able to build the docs without the dependencies
+ examples: Add libadwaita Leaflet sidebar example
+ examples: Port entry, actions and popover examples to 0.5

### Changed

+ core: Improved `DrawHandler`
+ core: Made the `macros` feature a default feature
+ core: Remove async-oneshot dependency and replace it with tokio's oneshot channel
+ core: Remove WidgetPlus in favor of RelmWidgetExt
+ core: Add convenience getter-methods to Controller
+ core: `add_action` of `RelmActionGroup` now takes a reference to a `RelmAction` as a parameter
+ examples: Many improvements
+ macros: `parse_with_path`, `update_stream`, `inject_view_code` and `generate_tokens` take references for some of their parameters
+ artwork: Update logo

### Fixed

+ macros: Fix usage of RelmContainerExt with local_ref attribute
+ macros: Report RelmContainerExt error at the correct span

## 0.5.0-beta.3 - 2022-9-28

### Added

+ core: Add `iter_mut` to `FactoryVecDeque`
+ core: Impl extension traits and `FactoryView` for `adw::PreferencesGroup`
+ core: Add a `prelude` module that contains commonly imported traits and types
+ core: Implement RelmContainerExt for Leaflet, Carousel and TabView
+ core: Add `iter()` method to `FactoryVecDeque`
+ core: Add getter for global application to simplify graceful shutdown of applications
+ core: Add MessageBroker type to allow communication between components on different levels
+ core: Return a clone of the `DynamicIndex` after inserting into a factory
+ macros: Add shorthand syntax for simple input messages
+ macros: Add chain attribute for properties
+ components: Add `SimpleComboBox` type as a more idiomatic wrapper around `gtk::ComboBoxText`
+ components: Port `OpenButton` to 0.5
+ book: Many chapters ported to 0.5

### Changed

+ core: Improve `SharedState` interface and prefer method names related to `RwLock`
+ core: Remove Debug requirement for FactoryComponent
+ core: Remove `input` and `output` fields on `ComponentSender` and `FactoryComponentSender` in favor of `input_sender` and `output_sender` methods
+ core: Make `ComponentSender` and `FactoryComponentSender` structs instead of type aliases
+ core: Increase MSRV to 1.63 to match the gtk4 crate
+ core: Rename `ParentMsg` and `output_to_parent_msg` to `ParentInput` and `output_to_parent_input`, respectively.
+ core: Do not call `gtk_init` and `adw_init` in favor of the application startup handler
+ core: Remove `Application` type alias in favor of `gtk::Application`
+ core: Make `app` field on `RelmApp` private
+ core: Use late initialization for transient_for and its native variant
+ core: Rename InitParams to Init in SimpleComponent and Worker too
+ macros: Don't generate dead code in the widgets struct
+ macros: Improve error reporting on invalid trait implementations

### Fixed

+ core: Append children for `gtk::Dialog` to its content area instead of using `set_child`
+ macros: Fix returned widgets assignment in the view macro

### Misc

+ all: Use more clippy lints and clean up the code in general

## 0.5.0-beta.2 - 2022-8-12

### Added

+ core: Add oneshot_command method to ComponentSender
+ core: Implement FactoryView for adw::Carousel
+ components: Complete port to 0.5
+ examples: More examples ported to 0.5

### Changed

+ core: Rename InitParams to Init
+ core: Pass senders by value
+ core: Make factories use FactoryComponentSender instead of individual senders for input and output
+ core: Remove generics from FactoryComponent

### Fixed

+ macros: Fix unsoundness with thread local memory

## 0.5.0-beta.1 - 2022-7-26

### Added

+ core: Introduce commands

### Changed

+ core: The Component trait replaces AppUpdate, ComponentUpdate, AsyncComponentUpdate, MessageHandler, MicroModel, MicroWidgets, Components and Widgets
+ core: Replace FactoryPrototype with FactoryComponent
+ core: Drop FactoryVec and make FactoryVecDeque easier to use
+ core: Improved component initialization and lifecycle
+ macros: Replace iterate, track and watch with attributes
+ macros: Replace args! with only parenthesis
+ macros: Improved macro syntax
+ examples: Many rewrites for the new version

## 0.4.4 - 2022-3-30

### Changed

+ all: Repositories were transferred to the Relm4 organization

### Fixed

+ macros: Don't interpret expr != expr as macro
+ core: Always initialize GTK/Libadwaita before running apps
+ macros: Some doc link fixes

## 0.4.3 - 2022-3-13

### Added

+ core: Add WidgetRef trait to make AsRef easier accessible for widgets
+ macros: Destructure widgets in Widgets::view

### Fixed

+ macros: Use correct widget type in derive macro for components
+ macros: Fix parsing of `property: value == other,` expressions
+ core: Fixed the position type for TabView
+ core: Fixed state changes in FactoryVec (by [V02460](https://github.com/V02460))
+ macros: Parse whole expressions instead of just literals

## 0.4.2 - 2022-2-4

### Added

+ macros: The view macro now allows dereferencing widgets with *

### Fixed

+ core: Fixed clear method of FactoryVec
+ macros: The micro_component macro now parses post_view correctly
+ macros: Fix the ordering of properties in the view macro
+ macros: Fix the ordering of widget assignments in the view macro

## 0.4.1 - 2022-1-17

### Added

+ macros: Improved documentation

### Fixed

+ core: Action macros now include the required traits themselves
+ macros: Allow connecting events in the view macro

## 0.4.0 - 2022-1-16

### Added

+ all: Update gtk4-rs to v0.4
+ core: Introduce the "macro" feature as alternative to using relm4-macros separately
+ macros: Add a macros for MicroComponents and Factories
+ macros: Add a post_view function to execute code after the view code of the macro
+ macros: Allow using the view and menu macros independently from the widget macro
+ macros: Allow using mutable widgets in view
+ macros: Improve error messages for anonymous widgets

### Changed

+ core: Renamed methods of the FactoryPrototype trait to better match with the rest of Relm4
+ macros: manual_view is now called pre_view
+ book: Reworked introduction and first chapter

### Fixed

+ core: Fix panic caused by the clear method of FactoryVecDeque

## 0.4.0-beta.3 - 2021-12-28

### Added

+ core: A factory view implementation for libadwaita's StackView
+ macros: Allow early returns in manual_view (by [euclio](https://github.com/euclio))

### Changed

+ core: Make GTK's command line argument handling optional (by [euclio](https://github.com/euclio))
+ core: DynamicIndex now implements Send but panics when used on other threads

## 0.4.0-beta.2 - 2021-11-26

+ macros: Add optional returned widget syntax

## 0.4.0-beta.1 - 2021-11-21

### Added

+ core: Micro components
+ core: Type safe actions API
+ macros: Menu macro for creating menus
+ macros: New returned widget syntax
+ examples Micro components example

### Changed

+ core: Initialize widgets from the outermost components to the app
+ macros: component! removed and parent! was added instead

### Removed

+ core: RelmComponent::with_new_thread

## 0.2.1 - 2021-10-17

### Added

+ core: Added sender method to RelmComponent
+ macros: New shorthand tracker syntax
+ macros: Allow generic function parameters in properties

### Changed

+ core: Use adw::Application when "libadwaita" feature is active

## 0.2.0 - 2021-10-09

### Changed

+ core: Pass model in connect_components function of the Widgets trait
+ core: Mini rework of factories
+ core: Removed DefaultWidgets trait in favor of Default implementations in gkt4-rs
+ book: Many book improvements by [tronta](https://github.com/tronta)

### Added

+ core: Added with_app method that allows passing an existing gtk::Application to Relm4
+ core: Methods to access the widgets of components
+ core: Re-export for gtk
+ macros: Support named arguments in the widget macro (by [mskorkowski](https://github.com/mskorkowski))
+ macros: Support usage of re-export paths in the widget macro (by [mskorkowski](https://github.com/mskorkowski))
+ macros: Added error message when confusing `=` and `:`
+ macros: Allow usage of visibilities other than pub
+ macros: New pre_connect_components and post_connect_components for manual components code

### Fixed

+ macros: Parsing the first widget should now always work as expected
+ macros: [#20](https://github.com/Relm4/relm4/issues/20) Fix wrong order when using components in the widget macro

## 0.1.0 - 2021-09-06

### Added

+ core: Added message handler type
+ core: More methods for factory data structures
+ macros: Add syntax for connecting events with components
+ examples: Stack example
+ book: Added macro expansion chapter

### Changed

+ book: Added message handler chapter and reworked the threads and async chapter
+ book: Many book improvements by [tronta](https://github.com/tronta)
+ core: The send! macro no longer clones the sender
+ macros: Make fields of public widgets public
+ components: Use &'static str instead of String for configurations
+ examples: Many improvements

### Fixed

+ macros: Use fully qualified syntax for factories
+ macros: Passing additional arguments now works for components and other properties, too.

## 0.1.0-beta.9 - 2021-08-24

### Added

+ components: Open button with automatic recent files list
+ components: Removed trait duplication and added more docs
+ core: Iterators added to factory data structures
+ core: More widgets added as FactoryView

### Changed

+ book: Many book improvements by [tronta](https://github.com/tronta)
+ core: Removed class name methods from WidgetPlus [#14](https://github.com/Relm4/relm4/pull/14)

### Fixed

+ macros: Parsing additional fields should be more stable now
+ macros: Widgets can not include comments at the top

## 0.1.0-beta.8 - 2021-08-20

### Added

+ core: Support for libadwaita 🎉
+ macros: Fully qualified syntax for trait disambiguation
+ macros: Allow passing additional arguments to widget initialization (useful e.g. for grids)
+ book: Reusable components and widget macro reference chapters

### Changed

+ macros: Improved error messages

## 0.1.0-beta.7 - 2021-08-19

### Added

+ book: Factory, components, worker and thread + async chapters

### Changed

+ core: get and get_mut of FactoryVec and FactoryVecDeque now return an Option to prevent panics

### Fixed

+ macros: Fixed components
+ core: Fixed unsound removal of elements in FactoryVecDeque


## 0.1.0-beta.6 - 2021-08-18

### Changed

+ core: Improved and adjusted the FactoryPrototype trait

### Added

+ core: Added the FactoryListView trait for more flexibility
+ core: Added a FactoryVecDeque container
+ core: Implemented FactoryView and FactoryListView for more widgets
+ examples: More examples

### Fixed

+ macros: Fixed the factory! macro

## 0.1.0-beta.5 - 2021-08-15

### Added

+ core: Drawing handler for gtk::DrawingArea
+ core: New CSS methods in WidgetPlus trait
+ examples: Many new examples

### Changed

+ core: Many doc improvements
+ macros: Improved tracker! macro
