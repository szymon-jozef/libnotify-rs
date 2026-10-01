use glib::MainLoop;
use libnotify_rs::{
    api::functions::{init, uninit},
    api::notification::Notification,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init("Coffee reminder")?;
    let mut notify = Notification::new("Drink coffee", "Click me when you're done drinking", None)?;

    // We need to have  a glib main loop in order to receive callback actions
    let main_loop = MainLoop::new(None, false);
    let loop_clone = main_loop.clone();

    notify.add_action("default", "I'm drinking!", move |action_name| {
        println!("User drunk his coffee!");
        println!("Action was named: {}", action_name);
        loop_clone.quit();
    })?;

    notify.show()?;

    main_loop.run();

    // Thanks to the main loop we can also read why the notification was closed
    println!(
        "Notification closed, because: {:?}",
        notify.get_closed_reason()
    );

    println!("Glib mainloop has ended");

    uninit();

    Ok(())
}
