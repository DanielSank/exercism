use std::collections::HashMap;

pub struct CircularBuffer<T> {
    contents: HashMap<usize, T>,
    cursor_write: Cursor,
    cursor_read: Cursor,
    capacity: usize,
}

#[derive(Eq)]
struct Cursor {
    val: usize,
    capacity: usize,
}

impl PartialEq for Cursor {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl From<&Cursor> for usize {
    fn from(item: &Cursor) -> Self {
        item.val
    }
}

impl Cursor {
    fn up(&mut self) {
        self.val +=1;
        if self.val == self.capacity {
            self.val = 0;
        }
    }
    fn down(&mut self) {
        self.val -=1;
        if self.val == usize::MAX {
            self.val = self.capacity - 1;
        }
    }

}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let contents: HashMap<usize, T> = HashMap::new();
        CircularBuffer {
            capacity,
            contents,
            cursor_write: Cursor { val: 0, capacity },
            cursor_read: Cursor { val: 0, capacity },
        }
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        if self.contents.len() == self.capacity {
            return Err(Error::FullBuffer)
        }
        self.contents.insert(usize::from(&self.cursor_write), _element);
        self.cursor_write.up();
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.contents.len() == 0 {
            return Err(Error::EmptyBuffer)
        }
        let to_return = self.contents.remove(&usize::from(&self.cursor_read));
        self.cursor_read.up();
        Ok(to_return.unwrap()) // fix
    }

    pub fn clear(&mut self) {
        self.cursor_write.val = 0;
        self.cursor_read.val = 0;
        self.contents.drain();
    }

    pub fn overwrite(&mut self, _element: T) {
        self.contents.insert(usize::from(&self.cursor_write), _element);
        if self.cursor_write == self.cursor_read {
            self.cursor_read.up();
        }
        self.cursor_write.up();
    }
}
