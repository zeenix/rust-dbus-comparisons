use super::MessageParts;
use gio::prelude::*;
use glib;

pub fn make_glib_message(parts: &MessageParts, send_it: bool) -> Option<Vec<u8>> {
    let struct_field = (parts.int2, parts.string2.as_str()).to_variant();
    let dict_variant = parts.dict.to_variant();
    let int_array_variant = parts.int_array.to_variant();
    let string_array: Vec<&str> = parts.string_array.iter().map(|s| s.as_str()).collect();
    let string_array_variant = string_array.to_variant();

    let mut elements = Vec::new();
    for _ in 0..parts.repeat {
        elements.push(parts.string1.as_str().to_variant());
        elements.push(parts.int1.to_variant());
        elements.push(struct_field.clone());
        elements.push(dict_variant.clone());
        elements.push(int_array_variant.clone());
        elements.push(string_array_variant.clone());
    }

    let body = glib::Variant::tuple_from_iter(elements);

    let msg = gio::DBusMessage::new_signal(&parts.object, &parts.interface, &parts.member);
    msg.set_body(&body);
    msg.set_serial(1);

    if send_it {
        let connection =
            gio::bus_get_sync(gio::BusType::Session, gio::Cancellable::NONE).unwrap();
        connection
            .send_message(&msg, gio::DBusSendMessageFlags::NONE)
            .unwrap();
        None
    } else {
        let blob = msg
            .to_blob(gio::DBusCapabilityFlags::NONE)
            .unwrap();
        Some(blob)
    }
}
