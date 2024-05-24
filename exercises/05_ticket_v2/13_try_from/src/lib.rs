// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for `Status`.
//  The parsing should be case-insensitive.

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TryFrom<String> for Status {
    type Error = std::fmt::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value {
            value if value.to_lowercase() == "todo" => Ok(Self::ToDo),
            value if value.to_lowercase() == "inprogress" => Ok(Self::InProgress),
            value if value.to_lowercase() == "done" => Ok(Self::Done),
            _ => Err(std::fmt::Error),
        }
    }
}

impl TryFrom<&str> for Status {
    type Error = std::fmt::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            value if value.to_lowercase() == "todo" => Ok(Self::ToDo),
            value if value.to_lowercase() == "inprogress" => Ok(Self::InProgress),
            value if value.to_lowercase() == "done" => Ok(Self::Done),
            _ => Err(std::fmt::Error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);
    }
}
