// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use crate::shared_generated::*;
use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_MESSAGE: u8 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_MESSAGE: u8 = 1;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_MESSAGE: [Message; 2] = [
  Message::NONE,
  Message::Pong,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct Message(pub u8);
#[allow(non_upper_case_globals)]
impl Message {
  pub const NONE: Self = Self(0);
  pub const Pong: Self = Self(1);

  pub const ENUM_MIN: u8 = 0;
  pub const ENUM_MAX: u8 = 1;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::NONE,
    Self::Pong,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::NONE => Some("NONE"),
      Self::Pong => Some("Pong"),
      _ => None,
    }
  }
}
impl core::fmt::Debug for Message {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for Message {
  type Inner = Self;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<u8>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for Message {
    type Output = Message;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        flatbuffers::emplace_scalar::<u8>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for Message {
  type Scalar = u8;
  #[inline]
  fn to_little_endian(self) -> u8 {
    self.0.to_le()
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(v: u8) -> Self {
    let b = u8::from_le(v);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for Message {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    u8::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for Message {}
pub struct MessageUnionTableOffset {}

pub enum PongOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Pong<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Pong<'a> {
  type Inner = Pong<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Pong<'a> {
  pub const VT_TIMESTAMP: flatbuffers::VOffsetT = 4;
  pub const VT_DATA: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Pong { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args PongArgs<'args>
  ) -> flatbuffers::WIPOffset<Pong<'bldr>> {
    let mut builder = PongBuilder::new(_fbb);
    builder.add_data(args.data);
    if let Some(x) = args.timestamp { builder.add_timestamp(x); }
    builder.finish()
  }


  #[inline]
  pub fn timestamp(&self) -> UnixTimestamp<'a> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<UnixTimestamp>>(Pong::VT_TIMESTAMP, None).unwrap()}
  }
  #[inline]
  pub fn data(&self) -> u64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u64>(Pong::VT_DATA, Some(0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for Pong<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<UnixTimestamp>>("timestamp", Self::VT_TIMESTAMP, true)?
     .visit_field::<u64>("data", Self::VT_DATA, false)?
     .finish();
    Ok(())
  }
}
pub struct PongArgs<'a> {
    pub timestamp: Option<flatbuffers::WIPOffset<UnixTimestamp<'a>>>,
    pub data: u64,
}
impl<'a> Default for PongArgs<'a> {
  #[inline]
  fn default() -> Self {
    PongArgs {
      timestamp: None, // required field
      data: 0,
    }
  }
}

pub struct PongBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> PongBuilder<'a, 'b> {
  #[inline]
  pub fn add_timestamp(&mut self, timestamp: flatbuffers::WIPOffset<UnixTimestamp<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<UnixTimestamp>>(Pong::VT_TIMESTAMP, timestamp);
  }
  #[inline]
  pub fn add_data(&mut self, data: u64) {
    self.fbb_.push_slot::<u64>(Pong::VT_DATA, data, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> PongBuilder<'a, 'b> {
    let start = _fbb.start_table();
    PongBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Pong<'a>> {
    let o = self.fbb_.end_table(self.start_);
    self.fbb_.required(o, Pong::VT_TIMESTAMP,"timestamp");
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Pong<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Pong");
      ds.field("timestamp", &self.timestamp());
      ds.field("data", &self.data());
      ds.finish()
  }
}
pub enum RootMessageOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct RootMessage<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for RootMessage<'a> {
  type Inner = RootMessage<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> RootMessage<'a> {
  pub const VT_MESSAGE_TYPE: flatbuffers::VOffsetT = 4;
  pub const VT_MESSAGE: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    RootMessage { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args RootMessageArgs
  ) -> flatbuffers::WIPOffset<RootMessage<'bldr>> {
    let mut builder = RootMessageBuilder::new(_fbb);
    if let Some(x) = args.message { builder.add_message(x); }
    builder.add_message_type(args.message_type);
    builder.finish()
  }


  #[inline]
  pub fn message_type(&self) -> Message {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<Message>(RootMessage::VT_MESSAGE_TYPE, Some(Message::NONE)).unwrap()}
  }
  #[inline]
  pub fn message(&self) -> flatbuffers::Table<'a> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(RootMessage::VT_MESSAGE, None).unwrap()}
  }
  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_pong(&self) -> Option<Pong<'a>> {
    if self.message_type() == Message::Pong {
      let u = self.message();
      // Safety:
      // Created from a valid Table for this object
      // Which contains a valid union in this slot
      Some(unsafe { Pong::init_from_table(u) })
    } else {
      None
    }
  }

}

impl flatbuffers::Verifiable for RootMessage<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_union::<Message, _>("message_type", Self::VT_MESSAGE_TYPE, "message", Self::VT_MESSAGE, true, |key, v, pos| {
        match key {
          Message::Pong => v.verify_union_variant::<flatbuffers::ForwardsUOffset<Pong>>("Message::Pong", pos),
          _ => Ok(()),
        }
     })?
     .finish();
    Ok(())
  }
}
pub struct RootMessageArgs {
    pub message_type: Message,
    pub message: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>,
}
impl<'a> Default for RootMessageArgs {
  #[inline]
  fn default() -> Self {
    RootMessageArgs {
      message_type: Message::NONE,
      message: None, // required field
    }
  }
}

pub struct RootMessageBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> RootMessageBuilder<'a, 'b> {
  #[inline]
  pub fn add_message_type(&mut self, message_type: Message) {
    self.fbb_.push_slot::<Message>(RootMessage::VT_MESSAGE_TYPE, message_type, Message::NONE);
  }
  #[inline]
  pub fn add_message(&mut self, message: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(RootMessage::VT_MESSAGE, message);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> RootMessageBuilder<'a, 'b> {
    let start = _fbb.start_table();
    RootMessageBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<RootMessage<'a>> {
    let o = self.fbb_.end_table(self.start_);
    self.fbb_.required(o, RootMessage::VT_MESSAGE,"message");
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for RootMessage<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("RootMessage");
      ds.field("message_type", &self.message_type());
      match self.message_type() {
        Message::Pong => {
          if let Some(x) = self.message_as_pong() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        _ => {
          let x: Option<()> = None;
          ds.field("message", &x)
        },
      };
      ds.finish()
  }
}
#[inline]
/// Verifies that a buffer of bytes contains a `RootMessage`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_root_message_unchecked`.
pub fn root_as_root_message(buf: &[u8]) -> Result<RootMessage, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<RootMessage>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `RootMessage` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_root_message_unchecked`.
pub fn size_prefixed_root_as_root_message(buf: &[u8]) -> Result<RootMessage, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<RootMessage>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `RootMessage` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_root_message_unchecked`.
pub fn root_as_root_message_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<RootMessage<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<RootMessage<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `RootMessage` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_root_message_unchecked`.
pub fn size_prefixed_root_as_root_message_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<RootMessage<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<RootMessage<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a RootMessage and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `RootMessage`.
pub unsafe fn root_as_root_message_unchecked(buf: &[u8]) -> RootMessage {
  flatbuffers::root_unchecked::<RootMessage>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed RootMessage and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `RootMessage`.
pub unsafe fn size_prefixed_root_as_root_message_unchecked(buf: &[u8]) -> RootMessage {
  flatbuffers::size_prefixed_root_unchecked::<RootMessage>(buf)
}
#[inline]
pub fn finish_root_message_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<RootMessage<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_root_message_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<RootMessage<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
