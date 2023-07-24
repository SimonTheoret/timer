
use notify_rust::{Notification, Timeout};
struct TimerNotification{
    notif: Notification
}
impl notification:
TimerNotification::new()
            .summary("Rust timer:")
            .body("Working period is done! \nTime to take a break")
            .appname("Rust timer")
            .timeout(Timeout::Never)
            .show()

