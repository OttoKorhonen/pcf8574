
pub enum Commands {
    DisplayAndCursorOn = 0x0F,
    ClearScreen = 0x01,
    ReturnCursorAtStart = 0x02,
    ShiftCursorLeft = 0x04,
    ShiftCursorRight = 0x06,
    ShiftDisplayRight = 0x05,
    ShiftDisplayLeft = 0x07,
    DisplayOnCursorBlinking = 0x0E,
    ForceCursorAtStart = 0x80,
    StartFromSecondLine = 0xC0,
    Form5x7Matrix = 0x38,
    SetCursorFirstLineThirdPosition = 0x83,
    ActivateSecondLine = 0x3C,
    DisplayAndCursorOff = 0x08,
    SetCursorAtSecondLineFirstPosition = 0xC1,
    DisplayOnWithNoVisibleCursor = 0x0C,
    SetCursorAtSecondLineSecondPosition = 0xC2
}
