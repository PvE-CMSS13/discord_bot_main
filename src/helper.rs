pub(crate) fn convert_to_safe_message(message: &str) -> String {
    let mut safe_message = message.replace("@here", "@\u{200B}here");
    safe_message = safe_message.replace("@everyone", "@\u{200B}everyone");

    return safe_message;
}
