//! "Marker" components: zero-sized structs which
//! can be used to filter for specific entities
//! in queries.

/// Zero-sized marker component used to mark players.
pub struct Player;

/// A player is in a gamemode where they may take damage.
pub struct CanTakeDamage;

/// A player is in a gamemode where they may instantly break blocks.
pub struct CanInstaBreak;

/// A player is allowed to break blocks.
pub struct CanBreak;

/// Marks that a player has teleported and
/// we should force-update the client's
/// position.
///
/// Only necessary for players.
pub struct Teleported;
