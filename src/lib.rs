//! Find out how to type a character by querying libcangjie's database.
//!
//! # Examples
//!
//! ```
//! # use std::error::Error;
//! #
//! use libcangjie_howtotype::{
//!     CangjieCode, CangjieVersion, LibCangjieHowToType
//! };
//! #
//! # fn main() -> Result<(), Box<dyn Error>> {
//! let cangjie = LibCangjieHowToType::new()?;
//!
//! let how_to_type = cangjie.how_to_type("喵", CangjieVersion::V3)?;
//! assert_eq!(*how_to_type, [CangjieCode::from_radicals("口廿田")]);
//! #
//! # Ok(())
//! # }
//! ```

use std::borrow::{Borrow, BorrowMut};
use std::fmt::{self, Display, Formatter};
use std::iter::Copied;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::slice::Iter;
use std::sync::LazyLock;

use rusqlite::types::ValueRef;
use rusqlite::{Connection, OpenFlags};
use smallvec::SmallVec;
use thiserror::Error;

pub use rusqlite;
pub use smallvec;

static DB_PATH: LazyLock<&Path> = LazyLock::new(|| Path::new("/usr/share/libcangjie/cangjie.db"));

/// Cangjie radical.
///
/// # Examples
///
/// ```
/// # use libcangjie_howtotype::CangjieRadical;
/// #
/// assert_eq!(CangjieRadical::from_code(b'a'), CangjieRadical::A);
/// assert_eq!(CangjieRadical::from_radical('日'), CangjieRadical::A);
/// assert_eq!(CangjieRadical::A.to_code(), b'a');
/// assert_eq!(CangjieRadical::A.to_radical(), '日');
/// ```
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum CangjieRadical {
    /// 日.
    A,
    /// 月.
    B,
    /// 金.
    C,
    /// 木.
    D,
    /// 水.
    E,
    /// 火.
    F,
    /// 土.
    G,
    /// 竹.
    H,
    /// 戈.
    I,
    /// 十.
    J,
    /// 大.
    K,
    /// 中.
    L,
    /// 一.
    M,
    /// 弓.
    N,
    /// 人.
    O,
    /// 心.
    P,
    /// 手.
    Q,
    /// 口.
    R,
    /// 尸.
    S,
    /// 廿.
    T,
    /// 山.
    U,
    /// 女.
    V,
    /// 田.
    W,
    /// 難.
    X,
    /// 卜.
    Y,
    /// Ｚ.
    ///
    /// This follows libcangjie's behaviour.
    Z,
}

impl CangjieRadical {
    /// Parses the code used by libcangjie (abcdefg…wxyz).
    ///
    /// # Panics
    ///
    /// Panics if the code is not a valid Cangjie radical code.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libcangjie_howtotype::CangjieRadical;
    /// #
    /// assert_eq!(CangjieRadical::from_code(b'a'), CangjieRadical::A);
    /// ```
    #[must_use]
    pub const fn from_code(code: u8) -> Self {
        match code {
            b'a' => Self::A,
            b'b' => Self::B,
            b'c' => Self::C,
            b'd' => Self::D,
            b'e' => Self::E,
            b'f' => Self::F,
            b'g' => Self::G,
            b'h' => Self::H,
            b'i' => Self::I,
            b'j' => Self::J,
            b'k' => Self::K,
            b'l' => Self::L,
            b'm' => Self::M,
            b'n' => Self::N,
            b'o' => Self::O,
            b'p' => Self::P,
            b'q' => Self::Q,
            b'r' => Self::R,
            b's' => Self::S,
            b't' => Self::T,
            b'u' => Self::U,
            b'v' => Self::V,
            b'w' => Self::W,
            b'x' => Self::X,
            b'y' => Self::Y,
            b'z' => Self::Z,
            _ => panic!("Invalid Cangjie radical code"),
        }
    }

    /// Parses the radical (日月金木水火土…田難卜Ｚ).
    ///
    /// Note that, following libcangjie's behaviour,
    /// the radical for `Z` is "Ｚ" instead of "重".
    ///
    /// # Panics
    ///
    /// Panics if the radical is not a valid Cangjie radical.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libcangjie_howtotype::CangjieRadical;
    /// #
    /// assert_eq!(CangjieRadical::from_radical('日'), CangjieRadical::A);
    /// ```
    ///
    /// Attempting to parse "重" will panic.
    ///
    /// ```should_panic
    /// # use libcangjie_howtotype::CangjieRadical;
    /// #
    /// let radical = CangjieRadical::from_radical('重');
    /// ```
    #[must_use]
    pub const fn from_radical(radical: char) -> Self {
        match radical {
            '日' => Self::A,
            '月' => Self::B,
            '金' => Self::C,
            '木' => Self::D,
            '水' => Self::E,
            '火' => Self::F,
            '土' => Self::G,
            '竹' => Self::H,
            '戈' => Self::I,
            '十' => Self::J,
            '大' => Self::K,
            '中' => Self::L,
            '一' => Self::M,
            '弓' => Self::N,
            '人' => Self::O,
            '心' => Self::P,
            '手' => Self::Q,
            '口' => Self::R,
            '尸' => Self::S,
            '廿' => Self::T,
            '山' => Self::U,
            '女' => Self::V,
            '田' => Self::W,
            '難' => Self::X,
            '卜' => Self::Y,
            'Ｚ' => Self::Z,
            _ => panic!("Invalid Cangjie radical"),
        }
    }

    /// Returns the code used by libcangjie (abcdefg…wxyz).
    ///
    /// # Examples
    ///
    /// ```
    /// # use libcangjie_howtotype::CangjieRadical;
    /// #
    /// assert_eq!(CangjieRadical::A.to_code(), b'a');
    /// ```
    #[must_use]
    pub const fn to_code(self) -> u8 {
        match self {
            Self::A => b'a',
            Self::B => b'b',
            Self::C => b'c',
            Self::D => b'd',
            Self::E => b'e',
            Self::F => b'f',
            Self::G => b'g',
            Self::H => b'h',
            Self::I => b'i',
            Self::J => b'j',
            Self::K => b'k',
            Self::L => b'l',
            Self::M => b'm',
            Self::N => b'n',
            Self::O => b'o',
            Self::P => b'p',
            Self::Q => b'q',
            Self::R => b'r',
            Self::S => b's',
            Self::T => b't',
            Self::U => b'u',
            Self::V => b'v',
            Self::W => b'w',
            Self::X => b'x',
            Self::Y => b'y',
            Self::Z => b'z',
        }
    }

    /// Returns the radical character (日月金木水火土…田難卜Ｚ).
    ///
    /// Note that, following libcangjie's behaviour,
    /// the radical for `Z` is "Ｚ" instead of "重".
    ///
    /// # Examples
    ///
    /// ```
    /// # use libcangjie_howtotype::CangjieRadical;
    /// #
    /// assert_eq!(CangjieRadical::A.to_radical(), '日');
    /// assert_eq!(CangjieRadical::Z.to_radical(), 'Ｚ');
    /// ```
    #[must_use]
    pub const fn to_radical(self) -> char {
        match self {
            Self::A => '日',
            Self::B => '月',
            Self::C => '金',
            Self::D => '木',
            Self::E => '水',
            Self::F => '火',
            Self::G => '土',
            Self::H => '竹',
            Self::I => '戈',
            Self::J => '十',
            Self::K => '大',
            Self::L => '中',
            Self::M => '一',
            Self::N => '弓',
            Self::O => '人',
            Self::P => '心',
            Self::Q => '手',
            Self::R => '口',
            Self::S => '尸',
            Self::T => '廿',
            Self::U => '山',
            Self::V => '女',
            Self::W => '田',
            Self::X => '難',
            Self::Y => '卜',
            Self::Z => 'Ｚ',
        }
    }
}

/// Code that can be used to type a character.
///
/// # Examples
///
/// ```
/// # use libcangjie_howtotype::{CangjieCode, CangjieRadical};
/// #
/// let code = CangjieCode::from(&[
///     CangjieRadical::R,
///     CangjieRadical::T,
///     CangjieRadical::W,
/// ][..]);
///
/// assert_eq!(CangjieCode::from_codes(b"rtw"), code);
/// assert_eq!(CangjieCode::from_radicals("口廿田"), code);
/// assert_eq!(code.codes().to_string(), "rtw");
/// assert_eq!(code.radicals().to_string(), "口廿田");
///
/// assert_eq!(code.len(), 3);
/// assert_eq!(code[1], CangjieRadical::T);
/// ```
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Default)]
pub struct CangjieCode(SmallVec<[CangjieRadical; 5]>);

impl CangjieCode {
    /// Parses a sequence of codes used by libcangjie (abcdefg…wxyz).
    ///
    /// # Panics
    ///
    /// Panics if any code in the sequence is not a valid Cangjie radical code.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libcangjie_howtotype::{CangjieCode, CangjieRadical};
    /// #
    /// assert_eq!(
    ///     CangjieCode::from_codes(b"rtw"),
    ///     CangjieCode::from(&[
    ///         CangjieRadical::R,
    ///         CangjieRadical::T,
    ///         CangjieRadical::W,
    ///     ][..]),
    /// );
    /// ```
    #[must_use]
    pub fn from_codes(codes: &[u8]) -> Self {
        codes
            .iter()
            .map(|&code| CangjieRadical::from_code(code))
            .collect()
    }

    /// Parses a sequence of radicals (日月金木水火土…田難卜Ｚ).
    ///
    /// Note that, following libcangjie's behaviour,
    /// the radical for `Z` is "Ｚ" instead of "重".
    ///
    /// # Panics
    ///
    /// Panics if any radical in the sequence is not a valid Cangjie radical.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libcangjie_howtotype::{CangjieCode, CangjieRadical};
    /// #
    /// assert_eq!(
    ///     CangjieCode::from_radicals("口廿田"),
    ///     CangjieCode::from(&[
    ///         CangjieRadical::R,
    ///         CangjieRadical::T,
    ///         CangjieRadical::W,
    ///     ][..]),
    /// );
    /// ```
    ///
    /// Attempting to parse "重" will panic.
    ///
    /// ```should_panic
    /// # use libcangjie_howtotype::CangjieCode;
    /// #
    /// let code = CangjieCode::from_radicals("重");
    /// ```
    #[must_use]
    pub fn from_radicals(radicals: &str) -> Self {
        radicals.chars().map(CangjieRadical::from_radical).collect()
    }

    /// Returns a display adapter for printing the codes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libcangjie_howtotype::{CangjieCode, CangjieRadical};
    /// #
    /// let code = CangjieCode::from(&[
    ///     CangjieRadical::R,
    ///     CangjieRadical::T,
    ///     CangjieRadical::W,
    /// ][..]);
    /// assert_eq!(code.codes().to_string(), "rtw");
    /// ```
    #[must_use]
    pub fn codes(&self) -> Codes<'_> {
        Codes(self)
    }

    /// Returns a display adapter for printing the radicals.
    ///
    /// Note that, following libcangjie's behaviour,
    /// the radical for `Z` is "Ｚ" instead of "重".
    ///
    /// # Examples
    ///
    /// ```
    /// # use libcangjie_howtotype::{CangjieCode, CangjieRadical};
    /// #
    /// let code = CangjieCode::from(&[
    ///     CangjieRadical::R,
    ///     CangjieRadical::T,
    ///     CangjieRadical::W,
    /// ][..]);
    /// assert_eq!(code.radicals().to_string(), "口廿田");
    ///
    /// assert_eq!(
    ///     CangjieCode::from(&[CangjieRadical::Z][..]).radicals().to_string(),
    ///     "Ｚ",
    /// );
    /// ```
    #[must_use]
    pub fn radicals(&self) -> Radicals<'_> {
        Radicals(self)
    }
}

impl Deref for CangjieCode {
    type Target = [CangjieRadical];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CangjieCode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<&[CangjieRadical]> for CangjieCode {
    fn from(value: &[CangjieRadical]) -> Self {
        Self(SmallVec::from(value))
    }
}

impl From<Vec<CangjieRadical>> for CangjieCode {
    fn from(value: Vec<CangjieRadical>) -> Self {
        Self(SmallVec::from(value))
    }
}

impl AsRef<[CangjieRadical]> for CangjieCode {
    fn as_ref(&self) -> &[CangjieRadical] {
        self.0.as_ref()
    }
}

impl AsMut<[CangjieRadical]> for CangjieCode {
    fn as_mut(&mut self) -> &mut [CangjieRadical] {
        self.0.as_mut()
    }
}

impl Borrow<[CangjieRadical]> for CangjieCode {
    fn borrow(&self) -> &[CangjieRadical] {
        self.0.borrow()
    }
}

impl BorrowMut<[CangjieRadical]> for CangjieCode {
    fn borrow_mut(&mut self) -> &mut [CangjieRadical] {
        self.0.borrow_mut()
    }
}

impl FromIterator<CangjieRadical> for CangjieCode {
    fn from_iter<T: IntoIterator<Item = CangjieRadical>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Extend<CangjieRadical> for CangjieCode {
    fn extend<T: IntoIterator<Item = CangjieRadical>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl IntoIterator for CangjieCode {
    type Item = CangjieRadical;
    type IntoIter = smallvec::IntoIter<[CangjieRadical; 5]>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a CangjieCode {
    type Item = CangjieRadical;
    type IntoIter = Copied<Iter<'a, CangjieRadical>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter().copied()
    }
}

/// A display adapter for printing the codes of a [`CangjieCode`].
#[derive(Debug)]
pub struct Codes<'a>(&'a CangjieCode);

impl Display for Codes<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for radical in self.0 {
            write!(f, "{}", char::from(radical.to_code()))?;
        }

        Ok(())
    }
}

/// A display adapter for printing the radicals of a [`CangjieCode`].
#[derive(Debug)]
pub struct Radicals<'a>(&'a CangjieCode);

impl Display for Radicals<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for radical in self.0 {
            write!(f, "{}", radical.to_radical())?;
        }

        Ok(())
    }
}

/// Cangjie version.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[non_exhaustive]
pub enum CangjieVersion {
    V3,
    V5,
}

/// The entrypoint of the library.
///
/// # Examples
///
/// ```
/// # use std::error::Error;
/// #
/// # use libcangjie_howtotype::{
/// #     CangjieCode, CangjieVersion, LibCangjieHowToType
/// # };
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let cangjie = LibCangjieHowToType::new()?;
///
/// let how_to_type = cangjie.how_to_type("喵", CangjieVersion::V3)?;
/// assert_eq!(*how_to_type, [CangjieCode::from_radicals("口廿田")]);
/// #
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct LibCangjieHowToType {
    db_conn: Connection,
}

impl LibCangjieHowToType {
    /// Creates a new `LibCangjieHowToType`.
    ///
    /// # Errors
    ///
    /// [`NewError::DBError`] if the database connection fails.
    pub fn new() -> NewResult<Self> {
        let db_conn = Connection::open_with_flags(
            *DB_PATH,
            OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )?;

        Ok(Self { db_conn })
    }

    /// Queries how to type a character.
    ///
    /// This method returns all possible ways to type the given character.
    /// If it doesn't know how to type the character,
    /// it returns an empty vector.
    ///
    /// Note that the capacity of the [`SmallVec`]
    /// is not part of the stable API.
    ///
    /// # Errors
    ///
    /// [`HowToTypeError::DBError`] if the database query fails.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// #
    /// # use libcangjie_howtotype::{
    /// #     CangjieCode, CangjieVersion, LibCangjieHowToType
    /// # };
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let cangjie = LibCangjieHowToType::new()?;
    ///
    /// let how_to_type = cangjie.how_to_type("喵", CangjieVersion::V3)?;
    /// assert_eq!(*how_to_type, [CangjieCode::from_radicals("口廿田")]);
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn how_to_type(
        &self,
        character: &str,
        version: CangjieVersion,
    ) -> HowToTypeResult<SmallVec<[CangjieCode; 1]>> {
        let version_num = match version {
            CangjieVersion::V3 => 3,
            CangjieVersion::V5 => 5,
        };

        let mut stmt = self.db_conn.prepare_cached(
            r"
                SELECT codes.code
                FROM chars
                JOIN codes
                  ON chars.char_index = codes.char_index
                WHERE chars.chchar = ?1 AND codes.version = ?2
            ",
        )?;
        let mut rows = stmt.query((character, version_num))?;

        let mut result = SmallVec::new();
        while let Some(row) = rows.next()? {
            let ValueRef::Text(code) = row.get_ref_unwrap(0) else {
                panic!("Unexpected value type")
            };
            let code = CangjieCode::from_codes(code);

            result.push(code);
        }

        Ok(result)
    }
}

/// Error type for [`LibCangjieHowToType::new`].
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum NewError {
    /// Database error.
    #[error("Database error")]
    DBError(#[from] rusqlite::Error),
}

/// A specialised [`Result`] type for [`LibCangjieHowToType::new`].
pub type NewResult<T> = Result<T, NewError>;

/// Error type for [`LibCangjieHowToType::how_to_type`].
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum HowToTypeError {
    /// Database error.
    #[error("Database error")]
    DBError(#[from] rusqlite::Error),
}

/// A specialised [`Result`] type for [`LibCangjieHowToType::how_to_type`].
pub type HowToTypeResult<T> = Result<T, HowToTypeError>;
