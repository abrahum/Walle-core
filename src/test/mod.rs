use crate::{
    action::*,
    alt::ColoredAlt,
    error::WalleError,
    event::*,
    segment::*,
    structs::Status,
    util::{Value, ValueMap},
    value, value_map,
};
use walle_macro::{_OneBot as OneBot, _PushToValueMap as PushToValueMap};

#[test]
fn event() {
    fn test<T>(event: (&str, Event, T))
    where
        T: TryFrom<Event, Error = WalleError> + std::fmt::Debug + PartialEq,
    {
        assert_eq!(serde_json::from_str::<Event>(event.0).unwrap(), event.1);
        assert_eq!(
            serde_json::from_str::<Event>(&serde_json::to_string(&event.1).unwrap()).unwrap(),
            event.1
        );
        assert_eq!(T::try_from(event.1.clone()).unwrap(), event.2);
        println!("{}", event.1.colored_alt());
    }

    test((
        r#"{
            "id": "b6e65187-5ac0-489c-b431-53078e9d2bbb",
            "impl": "go_onebot_qq",
            "platform": "qq",
            "self_id": "123234",
            "time": 1632847927.599013,
            "type": "meta",
            "detail_type": "heartbeat",
            "sub_type": "",
            "interval": 5000,
            "status": {
                "good": true,
                "online": true
            }
        }"#,
        Event {
            id: "b6e65187-5ac0-489c-b431-53078e9d2bbb".to_string(),
            implt: "go_onebot_qq".to_string(),
            platform: "qq".to_string(),
            self_id: "123234".to_string(),
            time: 1632847927.599013,
            ty: "meta".to_string(),
            detail_type: "heartbeat".to_string(),
            sub_type: "".to_string(),
            extra: value_map! {
                "interval": 5000,
                "status": {
                    "good": true,
                    "online": true
                }
            },
        },
        new_event(
            "b6e65187-5ac0-489c-b431-53078e9d2bbb".to_string(),
            1632847927.599013,
            "123234".to_string(),
            Meta {},
            Heartbeat {
                interval: 5000,
                status: Status {
                    good: true,
                    online: true,
                },
            },
            (),
            (),
            (),
            value_map!(),
        ),
    ));
    test((
        r#"{
            "id": "b6e65187-5ac0-489c-b431-53078e9d2bbb",
            "impl": "go_onebot_qq",
            "platform": "qq",
            "self_id": "123234",
            "time": 1632847927,
            "type": "message",
            "detail_type": "private",
            "sub_type": "",
            "message_id": "6283",
            "message": [
                {
                    "type": "text",
                    "data": {
                        "text": "OneBot is not a bot"
                    }
                },
                {
                    "type": "image",
                    "data": {
                        "file_id": "e30f9684-3d54-4f65-b2da-db291a477f16"
                    }
                }
            ],
            "alt_message": "OneBot is not a bot[??????]",
            "user_id": "123456788"
        }"#,
        Event {
            id: "b6e65187-5ac0-489c-b431-53078e9d2bbb".to_string(),
            implt: "go_onebot_qq".to_string(),
            platform: "qq".to_string(),
            self_id: "123234".to_string(),
            time: 1632847927.0,
            ty: "message".to_string(),
            detail_type: "private".to_string(),
            sub_type: "".to_string(),
            extra: value_map! {
                "message_id": "6283",
                "message": [
                    {
                        "type": "text",
                        "data": {
                            "text": "OneBot is not a bot"
                        }
                    },
                    {
                        "type": "image",
                        "data": {
                            "file_id": "e30f9684-3d54-4f65-b2da-db291a477f16"
                        }
                    }
                ],
                "alt_message": "OneBot is not a bot[??????]",
                "user_id": "123456788"
            },
        },
        new_event(
            "b6e65187-5ac0-489c-b431-53078e9d2bbb".to_string(),
            1632847927.0,
            "123234".to_string(),
            Message {
                message: vec![
                    crate::segment::MessageSegment {
                        ty: "text".to_string(),
                        data: value_map! {
                            "text": "OneBot is not a bot"
                        },
                    },
                    crate::segment::MessageSegment {
                        ty: "image".to_string(),
                        data: value_map! {
                            "file_id": "e30f9684-3d54-4f65-b2da-db291a477f16"
                        },
                    },
                ],
                message_id: "6283".to_string(),
                alt_message: "OneBot is not a bot[??????]".to_string(),
                user_id: "123456788".to_string(),
            },
            Private {},
            (),
            (),
            (),
            value_map!(),
        ),
    ));
}

#[test]
fn action() {
    fn test<T>(action: (&str, Action, T))
    where
        T: TryFrom<Action, Error = WalleError> + std::fmt::Debug + PartialEq,
    {
        assert_eq!(serde_json::from_str::<Action>(action.0).unwrap(), action.1);
        assert_eq!(
            serde_json::from_str::<Action>(&serde_json::to_string(&action.1).unwrap()).unwrap(),
            action.1
        );
        assert_eq!(T::try_from(action.1.clone()).unwrap(), action.2);
        println!("{}", action.1.colored_alt());
    }

    test((
        r#"{
            "action": "get_latest_events",
            "params": {
                "limit": 100,
                "timeout": 0
            }
        }"#,
        Action {
            action: "get_latest_events".to_string(),
            params: value_map! {
                "limit": 100,
                "timeout": 0
            },
        },
        GetLatestEvents {
            limit: 100,
            timeout: 0,
        },
    ));
    test((
        r#"{
            "action": "send_message",
            "params": {
                "detail_type": "group",
                "group_id": "12467",
                "message": [
                    {
                        "type": "text",
                        "data": {
                            "text": "??????????????????????????????"
                        }
                    }
                ]
            }
        }"#,
        Action {
            action: "send_message".to_string(),
            params: value_map! {
                "detail_type": "group",
                "group_id": "12467",
                "message": [
                    {
                        "type": "text",
                        "data": {
                            "text": "??????????????????????????????"
                        }
                    }
                ]
            },
        },
        SendMessage {
            detail_type: "group".to_string(),
            group_id: Some("12467".to_string()),
            message: vec![MessageSegment {
                ty: "text".to_string(),
                data: value_map! {
                    "text": "??????????????????????????????"
                },
            }],
            user_id: None,
            channel_id: None,
            guild_id: None,
        },
    ));
}

#[test]
fn action_resp() {}

#[test]
fn segment() {
    fn test<T>(message: (Value, MessageSegment, T))
    where
        T: TryFrom<MessageSegment, Error = WalleError> + std::fmt::Debug + PartialEq,
    {
        assert_eq!(MessageSegment::try_from(message.0).unwrap(), message.1);
        assert_eq!(T::try_from(message.1.clone()).unwrap(), message.2);
        println!("{}", message.1.alt());
    }

    test((
        value!({"type": "text",
            "data": {
                "text": "??????????????????????????????"
            }
        }),
        MessageSegment {
            ty: "text".to_string(),
            data: value_map! {
                "text": "??????????????????????????????"
            },
        },
        Text {
            text: "??????????????????????????????".to_string(),
        },
    ));
    test((
        value!({"type": "image",
            "data": {
                "file_id": "e30f9684-3d54-4f65-b2da-db291a477f16",
                "url": "https://example.com"
            }
        }),
        MessageSegment {
            ty: "image".to_string(),
            data: value_map! {
                "file_id": "e30f9684-3d54-4f65-b2da-db291a477f16",
                "url": "https://example.com"
            },
        },
        BaseSegment {
            segment: Image {
                file_id: "e30f9684-3d54-4f65-b2da-db291a477f16".to_string(),
            },
            extra: value_map! {
                "url": "https://example.com"
            },
        },
    ));
}

#[test]
fn valuemap_test() {
    let mut map = ValueMap::new();
    map.insert("key1".to_owned(), Value::Null);
    println!("{}", serde_json::to_string(&map).unwrap());
    let d = r#"{"key":3}"#;
    let map: ValueMap = serde_json::from_str(d).unwrap();
    println!("{:?}", map);
}

#[test]
fn enum_action() {
    #[derive(Debug, OneBot, PartialEq, Eq)]
    #[action]
    pub enum MyAction {
        GetUserInfo(GetUserInfo),
        GetGroupInfo { group_id: String },
    }

    let raw_action = Action {
        action: "get_user_info".to_string(),
        params: value_map! {
            "user_id": "abab"
        },
    };
    let action: MyAction = raw_action.try_into().unwrap();
    assert_eq!(
        action,
        MyAction::GetUserInfo(GetUserInfo {
            user_id: "abab".to_string()
        })
    );
}

#[test]
fn option_action() {
    #[derive(Debug, OneBot, PushToValueMap)]
    #[action]
    pub struct MySeg {
        pub text: Option<String>,
    }
    println!(
        "{:?}",
        MySeg::try_from(
            serde_json::from_str::<Action>(r#"{"action":"my_seg", "params": {"text": "text"}}"#)
                .unwrap()
        )
    );
    println!(
        "{:?}",
        MySeg::try_from(
            serde_json::from_str::<Action>(r#"{"action":"my_seg", "params": {}}"#).unwrap()
        )
    )
}
