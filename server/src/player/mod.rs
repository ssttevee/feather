//! Systems and components specific to player entities.

use crate::entity;
use crate::entity::NameComponent;
use crate::io::NewClientInfo;
use crate::network::Network;
use crate::state::State;
use mojang_api::ProfileProperty;

/// Profile properties of a player.
#[derive(Debug, Clone)]
pub struct ProfileProperties(pub Vec<ProfileProperty>);

/// Tag used to mark a player.
///
/// Note that this is a _tag_, not a component.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Player;

/// Creates a new player from the given `NewClientInfo`.
pub fn create(state: &State, info: NewClientInfo) {
    entity::base(state, info.position)
        .with_tag(Player)
        .with_component(info.uuid)
        .with_component(Network {
            sender: info.sender,
            receiver: info.receiver,
        })
        .with_component(info.ip)
        .with_component(ProfileProperties(info.profile))
        .with_component(NameComponent(info.username))
        .build();
}
