#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HeapPointer(pub usize);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConstantPointer(pub usize);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LocalPointer(pub usize);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StackPointer(usize);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InstructionPointer(pub usize);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TableIndex(usize);

impl InstructionPointer {
    pub fn increment(&mut self) {
        self.0 += 1
    }
    pub fn val(&self) -> usize {
        self.0
    }
}

impl From<usize> for HeapPointer {
    fn from(value: usize) -> Self {
        HeapPointer(value)
    }
}

impl Into<usize> for HeapPointer {
    fn into(self) -> usize {
        self.0
    }
}

impl From<usize> for TableIndex {
    fn from(value: usize) -> Self {
        TableIndex(value)
    }
}

impl Into<usize> for TableIndex {
    fn into(self) -> usize {
        self.0
    }
}

impl From<usize> for ConstantPointer {
    fn from(value: usize) -> Self {
        ConstantPointer(value)
    }
}
impl Into<usize> for ConstantPointer {
    fn into(self) -> usize {
        self.0
    }
}

impl From<usize> for LocalPointer {
    fn from(value: usize) -> Self {
        LocalPointer(value)
    }
}
impl Into<usize> for LocalPointer {
    fn into(self) -> usize {
        self.0
    }
}

impl From<usize> for StackPointer {
    fn from(value: usize) -> Self {
        StackPointer(value)
    }
}
impl Into<usize> for StackPointer {
    fn into(self) -> usize {
        self.0
    }
}

impl From<usize> for InstructionPointer {
    fn from(value: usize) -> Self {
        InstructionPointer(value)
    }
}
impl Into<usize> for InstructionPointer {
    fn into(self) -> usize {
        self.0
    }
}