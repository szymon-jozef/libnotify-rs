use libnotify_rs::{builder::NotificationBuilder, context::LibnotifyContext};

/// Builder is recommended way to interact with libnotify notifications

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = LibnotifyContext::new("Coffee addicted reminder")?; // builder requires context

    let notification_builder = NotificationBuilder::new(
        "Remember about your coffee!",
        "Only idiots forget about coffee!",
        None,
        &ctx,
    )?
    .set_urgency(libnotify_rs::api::notification::Urgency::Critical)
    .set_timeout(libnotify_rs::api::notification::Timeout::Never)
    .set_category("Coffee")?;

    notification_builder.show()?;

    // you don't need to uninitialize libnotify, cause context already does that!

    Ok(())
}
