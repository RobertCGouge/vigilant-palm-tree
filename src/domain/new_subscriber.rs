use crate::domain::subscriber_email::SubscriberEmail;
use crate::domain::SubscriberName;

pub struct NewSubscriber {
    // We are not using `String` anymore!
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}
