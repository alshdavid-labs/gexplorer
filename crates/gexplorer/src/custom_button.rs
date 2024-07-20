// use glib::Object;
// use gtk::glib;
// use gtk::subclass::prelude::*;

// // Object holding the state
// #[derive(Default)]
// pub struct CustomButtonInner;

// // The central trait for subclassing a GObject
// #[glib::object_subclass]
// impl ObjectSubclass for CustomButtonInner {
//   const NAME: &'static str = "MyGtkAppCustomButton";
//   type Type = self::CustomButton;
//   type ParentType = gtk::Button;
// }

// // Trait shared by all GObjects
// impl ObjectImpl for CustomButtonInner {}

// // Trait shared by all widgets
// impl WidgetImpl for CustomButtonInner {}

// // Trait shared by all buttons
// impl ButtonImpl for CustomButtonInner {}

// glib::wrapper! {
//   pub struct CustomButton(ObjectSubclass<self::CustomButtonInner>)
//       @extends gtk::Button, gtk::Widget,
//       @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
// }

// impl CustomButton {
//   pub fn new() -> Self {
//     Object::builder().build()
//   }

//   pub fn with_label(label: &str) -> Self {
//     Object::builder().property("label", label).build()
//   }
// }
// // ANCHOR_END: mod

// impl Default for CustomButton {
//   fn default() -> Self {
//     Self::new()
//   }
// }
