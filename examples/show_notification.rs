use libnotify_rs::api::{
    functions::{init, uninit},
    notification::{self, Notification},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init("I love coffee")?;
    let mut notify = Notification::new(
        "Remember to drink coffe!",
        "Coffee is very important for your mental health!",
        None,
    )?;

    notify.set_timeout(notification::Timeout::Custom(1000 * 10)); // 10 secs
    notify.set_urgency(notification::Urgency::Critical); // coffee is very important
    notify.set_category("Coffee")?;

    notify.show()?;

    uninit();

    Ok(())
}
