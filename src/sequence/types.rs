use bevy::prelude::*;

#[derive(Component, Debug, PartialEq, Eq, Hash)]
pub struct SequenceView {
    pub sequence: String,
}
