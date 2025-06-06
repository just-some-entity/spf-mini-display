pub trait FormatDuration 
{
    fn to_min_sec_str(&self) -> String;
}

impl FormatDuration for i32 
{
    fn to_min_sec_str(&self) -> String
    {
        let minutes = self / 60000;
        let seconds = (self % 60000) / 1000;
        format!("{}:{:02}", minutes, seconds)
    }
}