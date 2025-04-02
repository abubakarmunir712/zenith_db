use crate::enums::type_errors::DateTimeError;

pub struct TIME {
    hours: u8,
    minutes: u8,
    seconds: u8,
}

impl TIME {
    pub fn new(time: &str) -> Result<Self, DateTimeError> {
        let parts: Vec<&str> = time.split(":").collect();
        if parts.len() != 3 {
            return Err(DateTimeError::InvalidFormat);
        }
        let hours = parts[0];
        let minutes = parts[1];
        let seconds = parts[2];

        if hours.len() > 2 || minutes.len() > 2 || seconds.len() > 2 {
            return Err(DateTimeError::InvalidFormat);
        }

        let hours: u8 = hours.parse().map_err(|_| DateTimeError::InvalidFormat)?;
        let minutes: u8 = minutes.parse().map_err(|_| DateTimeError::InvalidFormat)?;
        let seconds: u8 = seconds.parse().map_err(|_| DateTimeError::InvalidFormat)?;

        if !Self::is_time_valid(hours, minutes, seconds) {
            return Err(DateTimeError::InvalidValue);
        }

        return Ok(Self {
            hours,
            minutes,
            seconds,
        });
    }

    pub fn is_time_valid(hours: u8, minutes: u8, seconds: u8) -> bool {
        if hours > 23 || minutes > 59 || seconds > 59 {
            return false;
        }
        return true;
    }

    pub fn to_bytes(&self) -> [u8; 3] {
        [self.hours, self.minutes, self.seconds]
    }

    pub fn from_bytes(bytes: &[u8; 3]) -> Self {
        TIME {
            hours: bytes[0],
            minutes: bytes[1],
            seconds: bytes[2],
        }
    }

    pub fn hours(&self) -> u8 {
        self.hours
    }
    pub fn minutes(&self) -> u8 {
        self.minutes
    }
    pub fn seconds(&self) -> u8 {
        self.seconds
    }

    pub fn value(&self) -> String {
        self.hours.to_string() + ":" + &self.minutes.to_string() + ":" + &self.seconds.to_string()
    }
}
