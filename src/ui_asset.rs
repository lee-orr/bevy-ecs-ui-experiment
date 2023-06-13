use bevy::{
    prelude::Name,
    reflect::{FromReflect, Reflect, TypeUuid},
};
use serde::{Deserialize, Serialize};

use crate::{string_expression::StringExpression, SimpleExpression};

#[derive(Debug, Clone, TypeUuid, Default, Reflect, FromReflect)]
#[uuid = "2c2788a6-ccfc-4f77-9c58-2f08c38e7ea0"]
pub struct UiNodeTree(pub Vec<UiNode>);

#[derive(Debug, Clone, Default, Reflect, FromReflect)]
pub enum UiNode {
    #[default]
    Empty,
    Node(Node<usize>),
    Image(Image),
    Text(Text),
    RawText(StringExpression),
    IfElse(IfElse),
}

impl UiNode {
    pub fn tag(&self) -> Name {
        match self {
            UiNode::Empty => Name::new("tg:empty"),
            UiNode::Node(_) => Name::new("tg:node"),
            UiNode::Image(_) => Name::new("tg:image"),
            UiNode::Text(_) => Name::new("tg:text"),
            UiNode::RawText(_) => Name::new("tg:raw_text"),
            UiNode::IfElse(_) => Name::new("tg:if_else"),
        }
    }
}

fn ui_node_intermediary_to_node_vec(
    vec: &mut Vec<UiNode>,
    node: &UiNodeIntermediary,
    previous_sibling: &Option<usize>,
) -> Option<usize> {
    match node {
        UiNodeIntermediary::Empty => None,
        UiNodeIntermediary::Node(n) => {
            let id = vec.len();
            vec.push(UiNode::Empty);
            let (_, children) = n.children.iter().fold(
                (None, Vec::with_capacity(n.children.len())),
                |(previous_child, mut children), child| {
                    let id = ui_node_intermediary_to_node_vec(vec, child, &previous_child);
                    let next_id = if let Some(id) = id {
                        children.push(id);
                        Some(id)
                    } else {
                        previous_child
                    };
                    (next_id, children)
                },
            );
            let node = Node {
                children,
                name: n.name.clone(),
                class: n.class.clone(),
                style: n.style.clone(),
            };
            let Some(n) = vec.get_mut(id) else { return None; };
            UiNode::Node(node).clone_into(n);
            Some(id)
        }
        UiNodeIntermediary::Image(img) => {
            let id = vec.len();
            vec.push(UiNode::Image(img.clone()));
            Some(id)
        }
        UiNodeIntermediary::Text(txt) => {
            let id = vec.len();
            vec.push(UiNode::Text(txt.clone()));
            Some(id)
        }
        UiNodeIntermediary::RawText(rtxt) => {
            let id = vec.len();
            vec.push(UiNode::RawText(rtxt.clone()));
            Some(id)
        }
        UiNodeIntermediary::If(IfElseTag { condition, child }) => {
            let if_id = vec.len();
            let Some(condition) = condition else { return None; };
            let id = if_id + 1;

            vec.push(UiNode::IfElse(IfElse {
                conditions: vec![(Some(condition.clone()), id)],
            }));
            ui_node_intermediary_to_node_vec(vec, child.as_ref(), &None);
            Some(if_id)
        }
        UiNodeIntermediary::Else(IfElseTag { condition, child }) => {
            let Some(previous_id) = previous_sibling else {return None;};
            let id = vec.len();
            let Some(previous) = vec.get_mut(*previous_id) else {
                return None;
            };
            let UiNode::IfElse(ref mut ifelse) = previous else {
                return None;
            };
            ifelse.conditions.push((condition.clone(), id));
            ui_node_intermediary_to_node_vec(vec, child.as_ref(), &None);
            None
        }
    }
}

impl<'de> Deserialize<'de> for UiNodeTree {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let intermediary = UiNodeIntermediary::deserialize(deserializer)?;
        let mut vec = vec![];
        ui_node_intermediary_to_node_vec(&mut vec, &intermediary, &None);
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
    If(IfElseTag),
    #[serde(rename = "else")]
    Else(IfElseTag),
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect, FromReflect)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Reflect, FromReflect)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Reflect, FromReflect)]
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

#[derive(Debug, Clone, Reflect, FromReflect)]
pub struct IfElse {
    pub conditions: Vec<(Option<SimpleExpression>, usize)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IfElseTag {
    #[serde(rename = "@condition")]
    pub condition: Option<SimpleExpression>,
    #[serde(rename = "$value")]
    pub child: Box<UiNodeIntermediary>,
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
        let UiNode::RawText(text) = parsed.0[*children.first().unwrap()].clone() else { panic!("Not a node") };
        assert_eq!(text.process(&NoContext), "some text");
    }

    #[test]
    fn can_deserialize_a_conditional_node() {
        let asset = r#"<if condition="1 + 2 == 3" ><node></node></if>"#;
        let parsed: UiNodeTree = from_str(asset).unwrap();
        let UiNode::IfElse(IfElse { conditions }) = parsed.0[0].clone() else { panic!("Not a node")};

        assert_eq!(conditions.len(), 1);

        let (Some(condition), child) = &conditions[0] else { panic!("No Condition")};

        let condition: bool = condition.process(&NoContext);
        assert!(condition);

        let UiNode::Node(Node { children: _, name: _, class: _, style: _ }) = parsed.0[*child].clone() else { panic!("Not a node") };
    }

    #[test]
    fn can_deserialize_a_conditional_node_with_else() {
        let asset =
            r#"<node><if condition="1 + 2 == 3"><node></node></if><else>test</else></node>"#;
        let parsed: UiNodeTree = from_str(asset).unwrap();
        let UiNode::IfElse(IfElse { conditions }) = parsed.0[1].clone() else { panic!("Not a node")};

        assert_eq!(conditions.len(), 2);

        let (Some(condition), child) = &conditions[0] else { panic!("No Condition")};

        let condition: bool = condition.process(&NoContext);
        assert!(condition);
        let UiNode::Node(Node { children: _, name: _, class: _, style: _ }) = parsed.0[*child].clone() else { panic!("Not a node") };

        let (None, child) = &conditions[1] else { panic!("False Condition Shouldn't Exist")};

        let UiNode::RawText(text) = parsed.0[*child].clone() else { panic!("Not a node") };

        assert_eq!(text.process(&NoContext), "test");
    }

    #[test]
    fn can_deserialize_a_conditional_node_with_else_if() {
        let asset = r#"<node><if condition="1 + 2 == 3"><node></node></if><else condition="1 + 2 == 4">run</else><else>test</else></node>"#;
        let parsed: UiNodeTree = from_str(asset).unwrap();

        assert_eq!(parsed.0.len(), 5);

        let UiNode::IfElse(IfElse { conditions }) = parsed.0[1].clone() else { panic!("Not a node")};

        assert_eq!(conditions.len(), 3);

        let (Some(condition), child) = &conditions[0] else { panic!("No Condition")};

        let condition: bool = condition.process(&NoContext);
        assert!(condition);
        let UiNode::Node(Node { children: _, name: _, class: _, style: _ }) = parsed.0[*child].clone() else { panic!("Not a node") };

        let (Some(condition), child) = &conditions[1] else { panic!("No Condition")};

        let condition: bool = condition.process(&NoContext);
        assert!(!condition);
        let UiNode::RawText(text) = parsed.0[*child].clone() else { panic!("Not a node") };

        assert_eq!(text.process(&NoContext), "run");

        let (None, child) = &conditions[2] else { panic!("False Condition Shouldn't Exist")};

        let UiNode::RawText(text) = parsed.0[*child].clone() else { panic!("Not a node") };

        assert_eq!(text.process(&NoContext), "test");
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

        let UiNode::Node(Node { name: _, class, style: _, children}) = &parsed.0[*children.first().unwrap()] else { panic!("Not a node")};
        assert_eq!(class.clone().unwrap().process(&NoContext), "class1 class2");

        {
            let UiNode::Image(Image { name, class, style: _, image_path}) = &parsed.0[*children.first().unwrap()] else { panic!("Not a node")};
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
