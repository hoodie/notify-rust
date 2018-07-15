extern crate notify_rust;

#[cfg(test)]
mod conversion{

    use notify_rust::NotificationUrgency as Urgency;

    #[test]
    fn urgency_from_str()
    {
        let u0:Urgency = "low".into();
        assert_eq!(u0, Urgency::Low);
        assert_eq!(Urgency::from("low"), Urgency::Low);
        assert_eq!(Urgency::from("medium"), Urgency::Normal);
        assert_eq!(Urgency::from("Normal"), Urgency::Normal);
        assert_eq!(Urgency::from("NoRmaL"), Urgency::Normal);
        assert_eq!(Urgency::from("High"), Urgency::Critical);
        assert_eq!(Urgency::from("Hi"), Urgency::Critical);
        assert_eq!(Urgency::from("Critical"), Urgency::Critical);
    }

}
