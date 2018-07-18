#[derive(Debug, PartialEq)]
pub enum Request {
    Send,
    Recieve,
}

#[derive(Debug, PartialEq)]
pub enum Message {
    Next(Request, usize),
    Prev(Request, usize),
    Clear(Request),
    Show(Request),
    Hide(Request),
    Index(Request, usize),
    IsVisible(Request, bool),
}

// need a duplex message type
