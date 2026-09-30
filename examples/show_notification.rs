use libnotify_rs::{
    functions::{init, uninit},
    notification::Notification,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init("I love coffee");
    let mut notify = Notification::new(
        "Remember to drink coffe!",
        "Coffee is very important for your mental health!",
        None,
    )?;

    notify.set_timeout(libnotify_rs::notification::Timeout::Custom(1000 * 10)); // 10 secs
    notify.set_urgency(libnotify_rs::notification::Urgency::Critical); // coffee is very important
    notify.set_category("Coffee")?;

    notify.show()?;

    uninit();

    Ok(())
}
