pub mod animation;
pub mod attachment;
pub mod bone;
pub mod camera;
pub mod color_animation;
pub mod event;
pub mod file_references;
pub mod infrastructure;
pub mod light;
pub mod material;
pub mod particle_emitter;
pub mod physics;
pub mod rendering_enhancements;
pub mod ribbon_emitter;
pub mod sequence;
pub mod texture;
pub mod texture_animation;
pub mod texture_transform;
pub mod transparency_animation;
pub mod vertex;

// Re-export common types
pub use attachment::{M2Attachment, M2AttachmentType};
pub use camera::{M2Camera, M2CameraFlags};
pub use color_animation::{M2Color, M2ColorAnimation};
pub use event::{M2Event, M2EventType};
pub use file_references::{
    AnimationFileIds, AnimationTrack, BoneData, BoneFileIds, BoneNode, CollisionMesh,
    InterpolationType, LodData, LodLevel, PhysicsData, PhysicsFileId, PhysicsMaterial,
    SkeletonData, SkeletonFileId, SkinFileIds, TextureFileIds,
};
pub use infrastructure::{ChunkHeader, ChunkReader};
pub use light::{M2Light, M2LightFlags, M2LightType};
pub use particle_emitter::{M2ParticleEmitter, M2ParticleEmitterType, M2ParticleFlags};
pub use physics::{
    M2PhysicsData, M2PhysicsFlags, M2PhysicsJoint, M2PhysicsShape, M2PhysicsShapeType,
};
pub use rendering_enhancements::{
    AdvancedParticleSystem, AfraChunk, AlphaBlendMode, BlendMode, CollisionFace, CollisionMaterial,
    CollisionMeshData, DbocChunk, DpivChunk, EdgeFadeData, EnhancedEmitter, ExtendedAnimationMode,
    ExtendedAnimationProperties, ExtendedEmitterProperties, ExtendedParticleData,
    ExtendedTextureAnimation, GeometryParticleIds, LightingDetails, LoopBehavior, ModelAlphaData,
    ParentAnimationBlacklist, ParentAnimationData, ParentEventData, ParentEventEntry,
    ParentSequenceBounds, ParticleGeosetData, ParticleGeosetEntry, ParticlePhysicsProperties,
    PhysicsFileDataChunk, PhysicsProperties, RecursiveParticleIds, SequenceBounds,
    TextureAnimationChunk, TextureBlendMode, TextureWeight, WaterfallEffect, WaterfallParameters,
};
pub use ribbon_emitter::M2RibbonEmitter;
pub use texture::{M2Texture, M2TextureFlags, M2TextureType};
pub use texture_animation::{M2TextureAnimation, M2TextureAnimationType};
pub use texture_transform::{C4Quaternion, M2TextureTransform, M2TextureTransformType};
pub use transparency_animation::M2TransparencyAnimation;
pub use vertex::M2Vertex;

/// Module for converting chunks between different versions
pub mod converter {
    use crate::error::Result;
    use crate::version::M2Version;

    /// Convert a chunk from one version to another
    pub trait VersionConverter {
        /// Convert self to a specified target version
        fn convert_to_version(&self, target_version: M2Version) -> Result<Self>
        where
            Self: Sized;
    }
}
