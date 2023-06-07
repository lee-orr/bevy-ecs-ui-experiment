use bevy::{
    reflect::{TypeUuid},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, TypeUuid)]
#[uuid = "2c2788a6-ccfc-4f77-9c58-2f08c38e7ea0"]
pub enum UiNode {
    #[serde(rename = "node")]
    Node(Node),
    #[serde(rename = "img")]
    Image(Image),
    #[serde(rename = "txt")]
    Text(Text),
    #[serde(rename = "$text")]
    RawText(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    #[serde(rename = "$value", default)]
    children: Vec<UiNode>,
    #[serde(rename = "@name")]
    name: Option<String>,
    #[serde(rename = "@class")]
    class: Option<String>,
    #[serde(rename = "@style")]
    style: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    #[serde(rename = "@name")]
    name: Option<String>,
    #[serde(rename = "@class")]
    class: Option<String>,
    #[serde(rename = "@style")]
    style: Option<String>,
    #[serde(rename = "@src")]
    image_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Text {
    #[serde(rename = "@name")]
    name: Option<String>,
    #[serde(rename = "@class")]
    class: Option<String>,
    #[serde(rename = "@style")]
    style: Option<String>,
    #[serde(alias = "@val", rename = "$value")]
    text: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use quick_xml::de::from_str;

    #[test]
    fn can_deserialize_a_single_node() {
        let asset = r#"<node></node>"#;
        let parsed: UiNode = from_str(asset).unwrap();
        let UiNode::Node(Node { name, class, style, children }) = parsed else { panic!("Not a node") };
        assert_eq!(children.len(), 0);
        assert!(name.is_none());
        assert!(class.is_none());
        assert!(style.is_none());
    }

    #[test]
    fn can_deserialize_a_named_node() {
        let asset = r#"<node name="test"></node>"#;
        let parsed: UiNode = from_str(asset).unwrap();
        let UiNode::Node(Node { name, class, style, children }) = parsed else { panic!("Not a node") };
        assert_eq!(children.len(), 0);
        assert_eq!(name.unwrap(), "test");
        assert!(class.is_none());
        assert!(style.is_none());
    }

    #[test]
    fn can_deserialize_a_node_with_a_class() {
        let asset = r#"<node class="test"></node>"#;
        let parsed: UiNode = from_str(asset).unwrap();
        let UiNode::Node(Node { name, class, style, children }) = parsed else { panic!("Not a node") };
        assert_eq!(children.len(), 0);
        assert_eq!(class.unwrap(), "test");
        assert!(name.is_none());
        assert!(style.is_none());
    }

    #[test]
    fn can_deserialize_a_styled_node() {
        let asset = r#"<node style="test"></node>"#;
        let parsed: UiNode = from_str(asset).unwrap();
        let UiNode::Node(Node { name, class, style, children }) = parsed else { panic!("Not a node") };
        assert_eq!(children.len(), 0);
        assert_eq!(style.unwrap(), "test");
        assert!(name.is_none());
        assert!(class.is_none());
    }

    #[test]
    fn can_deserialize_a_node_with_children() {
        let asset = r#"<node><node></node><node /></node>"#;
        let parsed: UiNode = from_str(asset).unwrap();
        let UiNode::Node(Node { name, class, style, children }) = parsed else { panic!("Not a node") };
        assert_eq!(children.len(), 2);
        assert!(name.is_none());
        assert!(class.is_none());
        assert!(style.is_none());
    }

    #[test]
    fn can_deserialize_an_image() {
        let asset = r#"<img src="test.png"></img>"#;
        let parsed: UiNode = from_str(asset).unwrap();
        let UiNode::Image(Image { name, class, style, image_path }) = parsed else { panic!("Not a node") };
        assert_eq!(image_path, "test.png");
        assert!(name.is_none());
        assert!(class.is_none());
        assert!(style.is_none());
    }

    #[test]
    fn can_deserialize_a_text() {
        let asset = r#"<txt val="some text"></txt>"#;
        let parsed: UiNode = from_str(asset).unwrap();
        let UiNode::Text(Text { name, class, style, text }) = parsed else { panic!("Not a node") };
        assert_eq!(text, "some text");
        assert!(name.is_none());
        assert!(class.is_none());
        assert!(style.is_none());
    }

    #[test]
    fn can_deserialize_a_text_value_as_child() {
        let asset = r#"<txt>some text</txt>"#;
        let parsed: UiNode = from_str(asset).unwrap();
        let UiNode::Text(Text { name, class, style, text }) = parsed else { panic!("Not a node") };
        assert_eq!(text, "some text");
        assert!(name.is_none());
        assert!(class.is_none());
        assert!(style.is_none());
    }

    #[test]
    fn can_deserialize_text_as_a_node_child() {
        let asset = r#"<node>some text</node>"#;
        let parsed: UiNode = from_str(asset).unwrap();
        let UiNode::Node(Node { name: _, class: _, style: _, children}) = parsed else { panic!("Not a node")};
        let UiNode::RawText(text) = children.get(0).unwrap() else { panic!("Not a node") };
        assert_eq!(text, "some text");
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
        let parsed: UiNode = from_str(asset).unwrap();
        let UiNode::Node(Node { name, class: _, style: _, children}) = parsed else { panic!("Not a node")};
        assert_eq!(name.unwrap(), "test");

        let UiNode::Node(Node { name: _, class, style: _, children}) = children.get(0).cloned().unwrap() else { panic!("Not a node")};
        assert_eq!(class.unwrap(), "class1 class2");

        {
            let UiNode::Image(Image { name, class, style: _, image_path}) = children.get(0).cloned().unwrap() else { panic!("Not a node")};
            assert_eq!(image_path, "test-image.png");
            assert_eq!(class.unwrap(), "img_class");
            assert_eq!(name.unwrap(), "image");
        }
        {
            let UiNode::RawText(text) = children.get(1).unwrap() else { panic!("not a text")};
            assert_eq!(text, "some raw text");
        }
        {
            let UiNode::Text(Text { name: _, class, style, text}) = children.get(2).cloned().unwrap() else { panic!("Not a node")};
            assert_eq!(text, "my text");
            assert_eq!(class.unwrap(), "text_class");
            assert_eq!(
                style.unwrap(),
                "font: libre-baskerville/LibreBaskerville-Regular.ttf;"
            );
        }
    }
}
