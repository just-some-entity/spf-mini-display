#[derive(Clone)]
pub struct TrackInfo
{
    pub title:          String,
    pub authors:        String,
    pub duration:       i32,
}

impl Default for TrackInfo
{
    fn default() -> Self
    {
        Self
        {
            title:    "N/A".to_string(),
            authors:  "N/A".to_string(),
            duration: 0,
        }
    }
}