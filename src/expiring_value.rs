use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct ExpiringValue<T>
{
    value : T,
    expired_value : T,
    last_modified: Instant,
    expiration_duration: Duration,
}

impl<T> ExpiringValue<T>
where T : Clone
{
    pub fn new(value : T, expiration_duration: Duration) -> Self
    {
        Self
        {
            value: value.clone(),
            expired_value: value.clone(),
            last_modified: Instant::now().checked_sub(expiration_duration).expect("???"),
            expiration_duration
        }
    }

    pub fn get(&self) -> T
    {
        match self.last_modified.elapsed() >= self.expiration_duration
        {
            true => { self.expired_value.clone() }
            false => { self.value.clone() }
        }
    }
    
    pub fn set(&mut self, value : T)
    {
        self.value = value;
        self.last_modified = Instant::now();
    }

    pub fn set_expired(&mut self, value : T)
    {
        self.expired_value = value;
    }
}