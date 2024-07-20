mod app_button;
mod custom_button;

use adw::Application;
use gtk::gio::MenuModel;
use gtk::glib;
use gtk::prelude::*;

const APP_ID: &str = "org.gtk_rs.HelloWorld1";

const CSS: &str = "
  .title {
    padding: 12px 8px 12px 12px;
    background-color: @headerbar_bg_color;
    // border-bottom: 1px solid @headerbar_shade_color;
  }

  .title .title-search  {
    margin-right: 6px;
  }

  .title .button-back,
  .title .button-forward,
  .title .title-address {
    margin-right: 8px;
  }

  .subtitle {
    min-height: 0px;
    // box-shadow: 1px 1px 2px rgba(0,0,0,0.1), -2px 2px 3px rgba(0,0,0,0.1);
    // border-bottom: 1px solid @secondary_sidebar_border_color;
    background-color: @headerbar_bg_color;
  }

  .sidebar {
    // border-right: 1px solid @secondary_sidebar_border_color;
    border: none;
    background-color: @secondary_sidebar_bg_color;
    cursor: pointer;
    position: relative;
  }

  .sidebar button {
    border-radius: 0;    
    background-color: @secondary_sidebar_bg_color;
  }

  .sidebar button:hover {
    background-color: @headerbar_bg_color;
  }
";

fn main() -> glib::ExitCode {
  let app = Application::builder()
    .application_id("com.davidalsh.example")
    .build();

  app.connect_startup(|_| load_css());
  app.connect_activate(build_ui);
  app.run()
}

fn load_css() {
  // Load the CSS file and add it to the provider
  let provider = gtk::CssProvider::new();
  provider.load_from_string(&CSS);

  // Add the provider to the default screen
  gtk::style_context_add_provider_for_display(
    &gtk::gdk::Display::default().expect("Could not connect to a display."),
    &provider,
    gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
  );
}

fn build_ui(app: &Application) {
  // let menu_button = gtk::MenuButton::builder()
  //   .icon_name("open-menu-symbolic")
  //   .build();
  // menu_button.add_css_class("button-menu");

  let title_bar = build_title_bar();
  let subtitle_bar = build_subtitle_bar();

  let body = gtk::Box::builder().build();
  body.add_css_class("body");
  body.set_orientation(gtk::Orientation::Vertical);
  body.append(&title_bar);
  // body.append(&subtitle_bar);

  let main = gtk::Paned::builder().build();
  main.add_css_class("main");
  main.set_vexpand(true);
  body.append(&main);

  let sidebar = gtk::Box::builder().build();
  sidebar.add_css_class("sidebar");
  sidebar.set_orientation(gtk::Orientation::Vertical);
  main.set_start_child(Some(&sidebar));
  main.set_position(250);
  // main.set_wide_handle(true);

  let defaults = ["Home", "Documents", "Pictures", "Desktop"];

  for item in defaults {
    let btn = gtk::Button::new();
    let label = gtk::Label::new(Some(item));
    label.set_xalign(0.0);
    btn.set_child(Some(&label));
    btn.set_cursor_from_name(Some("pointer"));
    sidebar.append(&btn);
  }

  let contents = gtk::Box::builder().build();
  contents.add_css_class("contents");
  main.set_end_child(Some(&contents));

  // let sidebar_inner = gtk::Box::builder().build();
  // sidebar_inner.add_css_class("sidebar-inner");
  // sidebar.append(&sidebar_inner);
  // sidebar_inner.set_width_request(205);

  // sidebar_inner.set_vexpand(true);

  // let sidebar_resize = gtk::Paned::builder().build();
  // sidebar_resize.add_css_class("sidebar-resize");
  // sidebar.append(&sidebar_resize);
  // sidebar_resize.set_cursor_from_name(Some("ew-resize"));
  // sidebar_resize.set_vexpand(true);

  // sidebar_resize.connect(gtk::SignalAction)

  let window = adw::Window::builder().content(&body).build();
  window.set_application(Some(app));

  window.set_default_height(800);
  window.set_default_width(1024);

  // window.set_child(Some(&body));
  window.present();
}

fn build_title_bar() -> gtk::Box {
  let title_bar = gtk::Box::builder().build();
  title_bar.add_css_class("title");

  let back_button = gtk::MenuButton::builder().icon_name("go-previous").build();
  back_button.add_css_class("button-back");
  back_button.set_cursor_from_name(Some("pointer"));

  let forward_button = gtk::MenuButton::builder().icon_name("go-next").build();
  forward_button.add_css_class("button-forward");
  forward_button.set_cursor_from_name(Some("pointer"));

  let address = gtk::Entry::builder().text("/home/dalsh").build();
  address.add_css_class("title-address");
  address.set_halign(gtk::Align::Fill);
  address.set_hexpand(true);

  let refresh_button = gtk::MenuButton::builder().icon_name("view-refresh").build();
  refresh_button.add_css_class("button-refresh");

  let up_button = gtk::MenuButton::builder().icon_name("go-up").build();
  up_button.add_css_class("button-up");

  let search = gtk::Entry::builder().placeholder_text("Search").build();
  search.add_css_class("title-search");

  let controls = gtk::WindowControls::builder().build();
  controls.set_side(gtk::PackType::End);

  title_bar.append(&back_button);
  title_bar.append(&forward_button);
  title_bar.append(&address);
  title_bar.append(&search);
  title_bar.append(&controls);

  title_bar
}

fn build_subtitle_bar() -> gtk::Box {
  let subtitle_bar = gtk::Box::builder().build();
  subtitle_bar.add_css_class("subtitle");

  let l = gtk::Label::new(Some(""));
  subtitle_bar.append(&l);
  subtitle_bar
}
