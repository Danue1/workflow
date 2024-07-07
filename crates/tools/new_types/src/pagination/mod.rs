use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct Pagination {
    cursor: Option<Uuid>,
    size: Size,
}

#[derive(Debug, Clone, Copy)]
pub struct Size(usize);

impl Pagination {
    #[inline]
    pub const fn cursor(&self) -> Option<Uuid> {
        self.cursor
    }

    #[inline]
    pub const fn size(&self) -> Size {
        self.size
    }
}

impl Size {
    pub fn new(value: usize) -> Self {
        Self(value.clamp(1, 50))
    }

    pub const fn as_usize(&self) -> usize {
        self.0
    }

    pub const fn as_i64(&self) -> i64 {
        self.0 as i64
    }
}

impl Default for Size {
    #[inline]
    fn default() -> Self {
        Self(20)
    }
}

impl From<usize> for Pagination {
    #[inline]
    fn from(size: usize) -> Self {
        Self {
            cursor: None,
            size: Size::new(size),
        }
    }
}

impl From<Option<usize>> for Pagination {
    #[inline]
    fn from(size: Option<usize>) -> Self {
        Self {
            cursor: None,
            size: match size {
                Some(size) => Size::new(size),
                None => Size::default(),
            },
        }
    }
}

impl From<(Uuid, usize)> for Pagination {
    #[inline]
    fn from((cursor, size): (Uuid, usize)) -> Self {
        Self {
            cursor: Some(cursor),
            size: Size::new(size),
        }
    }
}

impl From<Option<(Uuid, usize)>> for Pagination {
    #[inline]
    fn from(cursor_size: Option<(Uuid, usize)>) -> Self {
        match cursor_size {
            Some((cursor, size)) => Self {
                cursor: Some(cursor),
                size: Size::new(size),
            },
            None => Self {
                cursor: None,
                size: Size::default(),
            },
        }
    }
}

impl From<(Uuid, Option<usize>)> for Pagination {
    #[inline]
    fn from((cursor, size): (Uuid, Option<usize>)) -> Self {
        Self {
            cursor: Some(cursor),
            size: match size {
                Some(size) => Size::new(size),
                None => Size::default(),
            },
        }
    }
}

impl From<(Option<Uuid>, usize)> for Pagination {
    #[inline]
    fn from((cursor, size): (Option<Uuid>, usize)) -> Self {
        Self {
            cursor,
            size: Size::new(size),
        }
    }
}

impl From<(Option<Uuid>, Option<usize>)> for Pagination {
    #[inline]
    fn from((cursor, size): (Option<Uuid>, Option<usize>)) -> Self {
        Self {
            cursor,
            size: match size {
                Some(size) => Size::new(size),
                None => Size::default(),
            },
        }
    }
}

impl From<Size> for Pagination {
    #[inline]
    fn from(size: Size) -> Self {
        Self { cursor: None, size }
    }
}

impl From<Option<Size>> for Pagination {
    #[inline]
    fn from(size: Option<Size>) -> Self {
        Self {
            cursor: None,
            size: size.unwrap_or_default(),
        }
    }
}

impl From<(Uuid, Size)> for Pagination {
    #[inline]
    fn from((cursor, size): (Uuid, Size)) -> Self {
        Self {
            cursor: Some(cursor),
            size,
        }
    }
}

impl From<Option<(Uuid, Size)>> for Pagination {
    #[inline]
    fn from(cursor_size: Option<(Uuid, Size)>) -> Self {
        match cursor_size {
            Some((cursor, size)) => Self {
                cursor: Some(cursor),
                size,
            },
            None => Self {
                cursor: None,
                size: Size::default(),
            },
        }
    }
}

impl From<(Uuid, Option<Size>)> for Pagination {
    #[inline]
    fn from((cursor, size): (Uuid, Option<Size>)) -> Self {
        Self {
            cursor: Some(cursor),
            size: size.unwrap_or_default(),
        }
    }
}

impl From<(Option<Uuid>, Size)> for Pagination {
    #[inline]
    fn from((cursor, size): (Option<Uuid>, Size)) -> Self {
        Self { cursor, size }
    }
}

impl From<(Option<Uuid>, Option<Size>)> for Pagination {
    #[inline]
    fn from((cursor, size): (Option<Uuid>, Option<Size>)) -> Self {
        Self {
            cursor,
            size: size.unwrap_or_default(),
        }
    }
}
