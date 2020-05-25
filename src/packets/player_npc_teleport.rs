use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Player NPC Teleport.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct PlayerNPCTeleport {
    /// BitFlags: 0 = Player Teleport (Neither 1 or 2), 1 = NPC Teleport, 2 = Player Teleport to Other Player, 4 = GetPositionFromTarget, 8 = HasExtraInfo
    pub flags: u8,
    pub target_id: i16,
    pub x: i32 /* single */ ,
    pub y: i32 /* single */ ,
    pub style: u8,
    /// Only sent if HasExtraInfo flag is true
    pub extrainfo: i32,
}

impl PacketBody for PlayerNPCTeleport {
    const TAG: u8 = 65;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.flags);
        cursor.write(&self.target_id);
        cursor.write(&self.x);
        cursor.write(&self.y);
        cursor.write(&self.style);
        cursor.write(&self.extrainfo);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            flags: cursor.read(),
            target_id: cursor.read(),
            x: cursor.read(),
            y: cursor.read(),
            style: cursor.read(),
            extrainfo: cursor.read(),
        }
    }
}
