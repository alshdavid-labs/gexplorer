// use gtk::glib::Object;
// use rsx_gtk;
// // use xml;

// macro_rules! component {
//   ( $ ( $x:expr), * ) => {};
// }

// pub struct AppButton {}

// impl AppButton {
//   pub fn new() {
//     let doc = rsx_gtk::rsx!(
//       <Button>{"Click me"}</Button>
//     );

//     let mut element = Vec::<Object>::new();

//     let parser = xml::EventReader::new(doc.as_str().as_bytes());
//     let mut depth = 0;
//     for e in parser {
//       match e {
//         Ok(xml::reader::XmlEvent::StartElement { name, .. }) => {
//           println!("{:spaces$}+{name}", "", spaces = depth * 2);
//           depth += 1;
//           // if name == "Button" {}
//         }
//         Ok(xml::reader::XmlEvent::Characters(data)) => {
//           println!("{data}");
//         }
//         Ok(xml::reader::XmlEvent::EndElement { name }) => {
//           depth -= 1;
//           println!("{:spaces$}-{name}", "", spaces = depth * 2);
//         }
//         Err(e) => {
//           eprintln!("Error: {e}");
//           break;
//         }
//         // There's more: https://docs.rs/xml-rs/latest/xml/reader/enum.XmlEvent.html
//         _ => {}
//       }
//     }
//   }
// }
