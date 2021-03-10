use bytes::BytesMut;
use delegate_attr::delegate;
use postgres::types::{IsNull, ToSql, Type as SQLType};
use rocket::{http::RawStr, request::FromParam};
use rust_decimal::Decimal;
use serde::{Serialize, Serializer};
use std::{borrow::Cow, error::Error, num::ParseIntError, str::Utf8Error};

pub type MaybeUserName<'p> = Result<UserName, UserNameError<'p>>;

#[derive(Debug)]
pub struct UserName(Box<str>);

impl UserName {
	pub fn new(value: Box<str>) -> Result<Self, UserNameError<'static>> {
		if &*value == ""
			{return Err(UserNameError::Empty)}
		else if value.len() > 16
			{return Err(UserNameError::TooLarge(value))}
		else if value.trim() != &*value
			{return Err(UserNameError::TrailingWhiteSpace(value))}
		Ok(UserName(value))
	}

	#[allow(dead_code)]
	pub unsafe fn new_unchecked(value: Box<str>) -> Self {
		debug_assert_ne!(&*value, "");
		debug_assert_ne!(&*value, value.trim());
		debug_assert!(value.len() > 16);

		Self(value)
	}
}

impl<'p> FromParam<'p> for UserName {
	type Error = UserNameError<'p>;

	fn from_param(param: &'p RawStr) -> Result<Self, Self::Error> {
		let param = param.percent_decode()
			.map_err(|error| Self::Error::InvalidEncoding {error, param})?
			.into_owned().into_boxed_str();
		UserName::new(param)
	}
}

#[delegate(self.0)]
impl Serialize for UserName {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where S: Serializer;
}

impl ToSql for UserName {
	#[delegate((&*self.0 as &str))]
	fn to_sql(&self, ty: &SQLType, out: &mut BytesMut)
		-> Result<IsNull, Box<dyn Error + Sync + Send>> where Self: Sized;
	#[delegate((&*self.0 as &str))]
	fn to_sql_checked(&self, ty: &SQLType, out: &mut BytesMut)
		-> Result<IsNull, Box<dyn Error + Sync + Send>>;

	fn accepts(ty: &SQLType) -> bool
			where Self: Sized {
		<&str as ToSql>::accepts(ty)
	}
}

#[derive(Debug)]
pub enum UserNameError<'p> {
	InvalidEncoding {
		error: Utf8Error,
		param: &'p RawStr
	},
	Empty,
	TooLarge(Box<str>),
	TrailingWhiteSpace(Box<str>)
}

pub type MaybeDiscriminator<'p> = Result<Discriminator, DiscriminatorError<'p>>;

#[derive(Debug)]
#[rustc_layout_scalar_valid_range_end(9999)]
pub struct Discriminator(u16);

impl Discriminator {
	pub fn new(value: u16) -> Option<Self> {
		match value {
			// SAFETY: We just checked that the value is in range.
			0..=9999 => unsafe {Some(Self(value))},
			_ => None
		}
	}

	#[allow(dead_code)]
	pub unsafe fn new_unchecked(value: u16) -> Self {
		debug_assert!(9999 > value);

		// SAFETY: The burden of this contract is forwarded to the caller.
		unsafe {Self(value)}
	}
}

impl<'p> FromParam<'p> for Discriminator {
	type Error = DiscriminatorError<'p>;

	fn from_param(param: &'p RawStr) -> Result<Self, DiscriminatorError<'p>> {
		let value = param.percent_decode()
			.map_err(|error| Self::Error::InvalidEncoding {error, param})?;
		let value = param.parse()
			.map_err(|error| Self::Error::InvalidNumber {error, value})?;
		Self::new(value)
			.ok_or(Self::Error::OutOfRange(value))
	}
}

#[delegate(self.0)]
impl Serialize for Discriminator {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where S: Serializer;
}

impl ToSql for Discriminator {
	fn to_sql(&self, ty: &SQLType, out: &mut BytesMut)
			-> Result<IsNull, Box<dyn Error + Sync + Send>> where Self: Sized {
		Decimal::new(self.0 as i64, 0).to_sql(ty, out)
	}

	fn accepts(ty: &SQLType) -> bool
			where Self: Sized {
		*ty == SQLType::NUMERIC
	}

	fn to_sql_checked(&self, ty: &SQLType, out: &mut BytesMut)
			-> Result<IsNull, Box<dyn Error + Sync + Send>> {
		Decimal::new(self.0 as i64, 0).to_sql_checked(ty, out)
	}
}

#[derive(Debug)]
pub enum DiscriminatorError<'p> {
	InvalidEncoding {
		error: Utf8Error,
		param: &'p RawStr
	},
	InvalidNumber {
		error: ParseIntError,
		value: Cow<'p, str>
	},
	OutOfRange(u16)
}
