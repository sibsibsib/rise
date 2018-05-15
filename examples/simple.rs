#![feature(box_syntax)]
#![feature(proc_macro)]

extern crate rise;
extern crate rise_stylesheet;

use rise::{App, Layout, NodeContext, WindowOptions, WindowPosition};
use rise_stylesheet::styles::prelude::Stylesheet;
use std::boxed::Box;

use rise_stylesheet::yoga::Node;
use rise_stylesheet::yoga::NodeContextExt;

fn get_view_by_style(stylesheet: Stylesheet, style_name: &str) -> Node {
  let style = stylesheet.take(style_name.to_string()).unwrap();
  let mut context = NodeContext {
    data: Box::new(style_name.to_string()),
    style: Box::new(style),
  };

  let mut node = Node::new();
  context.style.apply_tag(&mut node, "default".to_string()).unwrap();
  node.set_context(Some(context));

  node
}

fn main() {
  let stylesheet = {
    let mut stylesheet = Stylesheet::default();
    stylesheet.load_from_string(include_str!("styles.json").to_string()).unwrap();
    stylesheet
  };

  let mut layout_container = get_view_by_style(stylesheet.clone(), "layout");

  let mut child = get_view_by_style(stylesheet.clone(), "greeze");
  layout_container.insert_child(&mut child, 0);

  let mut wheeze = get_view_by_style(stylesheet.clone(), "wheeze");
  layout_container.insert_child(&mut wheeze, 1);

  let app = App::new(
    WindowOptions {
      title: String::from("Example App"),
      position: WindowPosition::Center,
      window_size: (500, 500),
    },
    Layout::new(layout_container),
  );

  app.run();
}
