use bevy::reflect::TypeUuid;
use serde::{de::Visitor, Deserialize, Serialize};

use crate::{string_expression::StringExpression, SimpleExpression};

#[derive(Debug, Clone, TypeUuid, Default)]
#[uuid = "2c2788a6-ccfc-4f77-9c58-2f08c38e7ea0"]
pub struct UiNodeTree(pub Vec<UiNode>);

#[derive(Debug, Clone, Default)]
pub enum UiNode {
    #[default]
    Empty,
    Node(Node<usize>),
    Image(Image),
    Text(Text),
    RawText(StringExpression),
    Conditional(Conditional<usize>),
}

fn ui_node_intermediary_to_node_vec(vec: &mut Vec<UiNode>, node: UiNodeIntermediary) {
    match node {
        UiNodeIntermediary::Empty => vec.push(UiNode::Empty),
        UiNodeIntermediary::Node(n) => {
            let id = vec.len();
            vec.push(UiNode::Empty);
            let children = n
                .children
                .into_iter()
                .map(|child| {
                    let id = vec.len();
                    ui_node_intermediary_to_node_vec(vec, child);
                    id
                })
                .collect();
            let node = Node {
                children,
                name: n.name,
                class: n.class,
                style: n.style,
            };
            let Some(n) = vec.get_mut(id) else { return; };
            UiNode::Node(node).clone_into(n);
        }
        UiNodeIntermediary::Image(img) => vec.push(UiNode::Image(img)),
        UiNodeIntermediary::Text(txt) => vec.push(UiNode::Text(txt)),
        UiNodeIntermediary::RawText(rtxt) => vec.push(UiNode::RawText(rtxt)),
        UiNodeIntermediary::Conditional(_) => todo!(),
    }
}

impl<'de> Deserialize<'de> for UiNodeTree {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let intermediary = UiNodeIntermediary::deserialize(deserializer)?;
        let mut vec = vec![];
        ui_node_intermediary_to_node_vec(&mut vec, intermediary);
        Ok(UiNodeTree(vec))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum UiNodeIntermediary {
    #[default]
    #[serde(rename = "empty")]
    Empty,
    #[serde(rename = "node")]
    Node(Node<UiNodeIntermediary>),
    #[serde(rename = "img")]
    Image(Image),
    #[serde(rename = "txt")]
    Text(Text),
    #[serde(rename = "$text")]
    RawText(StringExpression),
    #[serde(rename = "if")]
    Conditional(Conditional<Box<UiNodeIntermediary>>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node<T: Default> {
    #[serde(rename = "$value", default)]
    pub children: Vec<T>,
    #[serde(rename = "@name")]
    pub name: Option<StringExpression>,
    #[serde(rename = "@class")]
    pub class: Option<StringExpression>,
    #[serde(rename = "@style")]
    pub style: Option<StringExpression>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    #[serde(rename = "@name")]
    pub name: Option<StringExpression>,
    #[serde(rename = "@class")]
    pub class: Option<StringExpression>,
    #[serde(rename = "@style")]
    pub style: Option<StringExpression>,
    #[serde(rename = "@src")]
    pub image_path: StringExpression,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Text {
    #[serde(rename = "@name")]
    pub name: Option<StringExpression>,
    #[serde(rename = "@class")]
    pub class: Option<StringExpression>,
    #[serde(rename = "@style")]
    pub style: Option<StringExpression>,
    #[serde(alias = "@val", rename = "$value")]
    pub text: StringExpression,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conditional<T> {
    #[serde(rename = "@condition")]
    pub condition: SimpleExpression,
    #[serde(rename = "true")]
    pub if_true: T,
    #[serde(rename = "false")]
    pub if_false: Option<T>,
}
#[cfg(test)]
mod test {
    use crate::Expression;

    use super::*;
    use bevy::{prelude::Component, reflect::Reflect};
    use quick_xml::de::from_str;

    #[derive(Component, Reflect)]
    pub struct NoContext;

    #[test]
    fn can_deserialize_a_single_node() {
        let asset = r#"<node></node>"#;
        let parsed: UiNodeTree = from_str(asset).unwrap();
        let UiNode::Node(Node { name, class, style, children }) = parsed.0[0].clone() else { panic!("Not a node") };
        assert_eq!(children.len(), 0);
        assert!(name.is_none());
        assert!(class.is_none());
        assert!(style.is_none());
    }

    #[test]
    fn can_deserialize_a_named_node() {
        let asset: &str = r#"<node name="test"></node>"#;
        let parsed: UiNodeTree = from_str(asset).unwrap();
        let UiNode::Node(Node { name, class, style, children }) = parsed.0[0].clone() else { panic!("Not a node") };
        assert_eq!(children.len(), 0);
        assert_eq!(name.unwrap().process(&NoContext), "test");
        assert!(class.is_none());
        assert!(style.is_none());
    }

    #[test]
    fn can_deserialize_a_node_with_a_class() {
        let asset = r#"<node class="test"></node>"#;
        let parsed: UiNodeTree = from_str(asset).unwrap();
        let UiNode::Node(Node { name, class, style, children }) = parsed.0[0].clone() else { panic!("Not a node") };
        assert_eq!(children.len(), 0);
        assert_eq!(class.unwrap().process(&NoContext), "test");
        assert!(name.is_none());
        assert!(style.is_none());
    }

    #[test]
    fn can_deserialize_a_styled_node() {
        let asset = r#"<node style="test"></node>"#;
        let parsed: UiNodeTree = from_str(asset).unwrap();
        let UiNode::Node(Node { name, class, style, children }) = parsed.0[0].clone() else { panic!("Not a node") };
        assert_eq!(children.len(), 0);
        assert_eq!(style.unwrap().process(&NoContext), "test");
        assert!(name.is_none());
        assert!(class.is_none());
    }

    #[test]
    fn can_deserialize_a_node_with_children() {
        let asset = r#"<node><node></node><node /></node>"#;
        let parsed: UiNodeTree = from_str(asset).unwrap();
        let UiNode::Node(Node { name, class, style, children }) = parsed.0[0].clone() else { panic!("Not a node") };
        assert_eq!(children.len(), 2);
        assert!(name.is_none());
        assert!(class.is_none());
        assert!(style.is_none());
    }

    #[test]
    fn can_deserialize_an_image() {
        let asset = r#"<img src="test.png"></img>"#;
        let parsed: UiNodeTree = from_str(asset).unwrap();
        let UiNode::Image(Image { name, class, style, image_path }) = parsed.0[0].clone() else { panic!("Not a node") };
        assert_eq!(image_path.process(&NoContext), "test.png");
        assert!(name.is_none());
        assert!(class.is_none());
        assert!(style.is_none());
    }

    #[test]
    fn can_deserialize_a_text() {
        let asset = r#"<txt val="some text"></txt>"#;
        let parsed: UiNodeTree = from_str(asset).unwrap();
        let UiNode::Text(Text { name, class, style, text }) = parsed.0[0].clone() else { panic!("Not a node") };
        assert_eq!(text.process(&NoContext), "some text");
        assert!(name.is_none());
        assert!(class.is_none());
        assert!(style.is_none());
    }

    #[test]
    fn can_deserialize_a_text_value_as_child() {
        let asset = r#"<txt>some text</txt>"#;
        let parsed: UiNodeTree = from_str(asset).unwrap();
        let UiNode::Text(Text { name, class, style, text }) = parsed.0[0].clone() else { panic!("Not a node") };
        assert_eq!(text.process(&NoContext), "some text");
        assert!(name.is_none());
        assert!(class.is_none());
        assert!(style.is_none());
    }

    #[test]
    fn can_deserialize_text_as_a_node_child() {
        let asset = r#"<node>some text</node>"#;
        let parsed: UiNodeTree = from_str(asset).unwrap();
        let UiNode::Node(Node { name: _, class: _, style: _, children}) = parsed.0[0].clone() else { panic!("Not a node")};
        let UiNode::RawText(text) = parsed.0[*children.get(0).unwrap()].clone() else { panic!("Not a node") };
        assert_eq!(text.process(&NoContext), "some text");
    }

    #[test]
    fn can_deserialize_simple_asset() {
        let asset = r#"
        <node name="test">
            <node class="class1 class2">
                <img class="img_class" name="image" src="test-image.png"/>
                some raw text
                <txt class="text_class" style="font: libre-baskerville/LibreBaskerville-Regular.ttf;">my text</txt>
            </node>
        </node>
        "#;
        let parsed: UiNodeTree = from_str(asset).unwrap();
        let UiNode::Node(Node { name, class: _, style: _, children}) = &parsed.0[0].clone() else { panic!("Not a node")};
        assert_eq!(name.clone().unwrap().process(&NoContext), "test");

        let UiNode::Node(Node { name: _, class, style: _, children}) = &parsed.0[*children.get(0).unwrap()] else { panic!("Not a node")};
        assert_eq!(class.clone().unwrap().process(&NoContext), "class1 class2");

        {
            let UiNode::Image(Image { name, class, style: _, image_path}) = &parsed.0[*children.get(0).unwrap()] else { panic!("Not a node")};
            assert_eq!(image_path.process(&NoContext), "test-image.png");
            assert_eq!(class.clone().unwrap().process(&NoContext), "img_class");
            assert_eq!(name.clone().unwrap().process(&NoContext), "image");
        }
        {
            let UiNode::RawText(text) = &parsed.0[*children.get(1).unwrap()] else { panic!("not a text")};
            assert_eq!(text.clone().process(&NoContext), "some raw text");
        }
        {
            let UiNode::Text(Text { name: _, class, style, text}) = &parsed.0[*children.get(2).unwrap()] else { panic!("Not a node")};
            assert_eq!(text.process(&NoContext), "my text");
            assert_eq!(class.clone().unwrap().process(&NoContext), "text_class");
            assert_eq!(
                style.clone().unwrap().process(&NoContext),
                "font: libre-baskerville/LibreBaskerville-Regular.ttf;"
            );
        }
    }
}
