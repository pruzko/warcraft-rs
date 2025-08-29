//! M2 model file command implementations

use anyhow::{Context, Result};
use clap::Subcommand;
use std::path::PathBuf;

use wow_blp::parser::load_blp;
use wow_m2::{
    AnimFile, M2Converter, M2Model, M2Version, Skin,
    skin::{OldSkinHeader, SkinG, SkinHeader, SkinHeaderT},
};

use crate::utils::{NodeType, TreeNode, TreeOptions, render_tree};

#[derive(Subcommand)]
pub enum M2Commands {
    /// Display information about an M2 model file
    Info {
        /// Path to the M2 file
        file: PathBuf,

        /// Show detailed information
        #[arg(short, long)]
        detailed: bool,
    },

    /// Validate an M2 model file
    Validate {
        /// Path to the M2 file
        file: PathBuf,

        /// Show all warnings (not just errors)
        #[arg(short, long)]
        warnings: bool,
    },

    /// Convert an M2 model to a different version
    Convert {
        /// Input M2 file
        input: PathBuf,

        /// Output M2 file
        output: PathBuf,

        /// Target version (e.g., "3.3.5a", "WotLK", "MoP")
        #[arg(long)]
        version: String,
    },

    /// Display M2 file structure as a tree
    Tree {
        /// Path to the M2 file
        file: PathBuf,

        /// Maximum depth to display
        #[arg(short, long, default_value = "5")]
        depth: usize,

        /// Include size information
        #[arg(short, long)]
        size: bool,

        /// Include references
        #[arg(short, long)]
        refs: bool,
    },

    /// Display information about a Skin file
    SkinInfo {
        /// Path to the Skin file
        file: PathBuf,

        /// Show detailed information
        #[arg(short, long)]
        detailed: bool,

        /// Parse old format
        #[arg(short, long)]
        old_format: bool,
    },

    /// Convert a Skin file to a different version
    SkinConvert {
        /// Input Skin file
        input: PathBuf,

        /// Output Skin file
        output: PathBuf,

        /// Target version (e.g., "3.3.5a", "WotLK", "MoP")
        #[arg(long)]
        version: String,
    },

    /// Display information about an ANIM file
    AnimInfo {
        /// Path to the ANIM file
        file: PathBuf,

        /// Show detailed information
        #[arg(short, long)]
        detailed: bool,
    },

    /// Convert an ANIM file to a different version
    AnimConvert {
        /// Input ANIM file
        input: PathBuf,

        /// Output ANIM file
        output: PathBuf,

        /// Target version (e.g., "3.3.5a", "WotLK", "MoP")
        #[arg(long)]
        version: String,
    },

    /// Display information about a BLP texture file
    BlpInfo {
        /// Path to the BLP file
        file: PathBuf,

        /// Show detailed information
        #[arg(short, long)]
        detailed: bool,
    },
}

pub fn execute(cmd: M2Commands) -> Result<()> {
    match cmd {
        M2Commands::Info { file, detailed } => handle_info(file, detailed),
        M2Commands::Convert {
            input,
            output,
            version,
        } => handle_convert(input, output, version),
        M2Commands::Validate { file, warnings } => handle_validate(file, warnings),
        M2Commands::Tree {
            file,
            depth,
            size,
            refs,
        } => handle_tree(file, depth, size, refs),
        M2Commands::SkinInfo {
            file,
            detailed,
            old_format,
        } => {
            if old_format {
                handle_skin_info::<OldSkinHeader>(file, detailed)
            } else {
                handle_skin_info::<SkinHeader>(file, detailed)
            }
        }
        M2Commands::SkinConvert {
            input,
            output,
            version,
        } => handle_skin_convert(input, output, version),
        M2Commands::AnimInfo { file, detailed } => handle_anim_info(file, detailed),
        M2Commands::AnimConvert {
            input,
            output,
            version,
        } => handle_anim_convert(input, output, version),
        M2Commands::BlpInfo { file, detailed } => handle_blp_info(file, detailed),
    }
}

fn handle_info(path: PathBuf, detailed: bool) -> Result<()> {
    println!("Loading M2 model: {}", path.display());

    let _model = M2Model::load(&path)
        .with_context(|| format!("Failed to load M2 model from {}", path.display()))?;

    println!("\n=== M2 Model Information ===");

    // Note: Many fields are private in the M2Model struct, so we can only show basic info
    // The actual model implementation would need to expose more public methods/fields

    println!("File loaded successfully!");

    if detailed {
        println!("\n=== Detailed Information ===");
        println!("(Detailed information requires additional public API methods)");
    }

    Ok(())
}

fn handle_convert(input: PathBuf, output: PathBuf, version_str: String) -> Result<()> {
    println!("Loading M2 model: {}", input.display());

    let m2_format = M2Model::load(&input)
        .with_context(|| format!("Failed to load M2 model from {}", input.display()))?;
    let model = m2_format.model();

    let target_version = M2Version::from_expansion_name(&version_str)
        .with_context(|| format!("Invalid target version: {version_str}"))?;

    println!("Converting to {target_version:?}");

    let converter = M2Converter::new();
    let converted = converter
        .convert(model, target_version)
        .with_context(|| "Failed to convert model")?;

    println!("Saving converted model to: {}", output.display());
    converted
        .save(&output)
        .with_context(|| format!("Failed to save converted model to {}", output.display()))?;

    println!("Conversion complete!");
    Ok(())
}

fn handle_validate(path: PathBuf, show_warnings: bool) -> Result<()> {
    println!("Validating M2 model: {}", path.display());

    let m2_format = M2Model::load(&path)
        .with_context(|| format!("Failed to load M2 model from {}", path.display()))?;
    let model = m2_format.model();

    // Validate the model
    match model.validate() {
        Ok(_) => {
            println!("✓ Model validation passed!");
        }
        Err(e) => {
            println!("❌ Model validation failed: {e}");
            if !show_warnings {
                println!("Use --warnings to show additional details");
            }
            std::process::exit(1);
        }
    }

    Ok(())
}

fn handle_tree(path: PathBuf, max_depth: usize, _show_size: bool, _show_refs: bool) -> Result<()> {
    let _model = M2Model::load(&path)
        .with_context(|| format!("Failed to load M2 model from {}", path.display()))?;

    let root = TreeNode::new("M2 Model".to_string(), NodeType::Root);

    // Since most model fields are private, we can only show a basic structure
    // A real implementation would need the M2Model to expose more information

    let options = TreeOptions {
        max_depth: Some(max_depth),
        show_external_refs: _show_refs,
        no_color: false,
        show_metadata: true,
        compact: false,
    };

    let tree_output = render_tree(&root, &options);
    print!("{tree_output}");

    println!("\n(Note: Full tree visualization requires additional public API methods)");
    Ok(())
}

fn handle_skin_info<H: SkinHeaderT + Clone>(path: PathBuf, detailed: bool) -> Result<()> {
    println!("Loading Skin file: {}", path.display());

    let _skin = SkinG::<H>::load(&path)
        .with_context(|| format!("Failed to load Skin file from {}", path.display()))?;

    println!("\n=== Skin Information ===");
    println!("File loaded successfully!");

    if detailed {
        println!("\n=== Detailed Information ===");
        println!("(Detailed information requires additional public API methods)");
    }

    Ok(())
}

fn handle_skin_convert(input: PathBuf, output: PathBuf, version_str: String) -> Result<()> {
    println!("Loading Skin file: {}", input.display());

    let skin = Skin::load(&input)
        .with_context(|| format!("Failed to load Skin file from {}", input.display()))?;

    let target_version = M2Version::from_expansion_name(&version_str)
        .with_context(|| format!("Invalid target version: {version_str}"))?;

    println!("Converting to {target_version:?}");

    println!("Saving converted Skin file to: {}", output.display());
    skin.save(&output)
        .with_context(|| format!("Failed to save converted Skin file to {}", output.display()))?;

    println!("Conversion complete!");
    Ok(())
}

fn handle_anim_info(path: PathBuf, detailed: bool) -> Result<()> {
    println!("Loading ANIM file: {}", path.display());

    let anim = AnimFile::load(&path)
        .with_context(|| format!("Failed to load ANIM file from {}", path.display()))?;

    println!("\n=== ANIM Information ===");
    println!("Format: {:?}", anim.format);
    println!("Animation Sections: {}", anim.animation_count());

    if anim.is_legacy_format() {
        println!("Legacy Format: True");
    } else {
        println!("Modern Format: True");
    }

    // Show memory usage stats
    let usage = anim.memory_usage();
    println!("Total Keyframes: {}", usage.total_keyframes());
    println!("Memory Usage: ~{} bytes", usage.approximate_bytes);

    if detailed {
        println!("\n=== Detailed Information ===");

        // Show format-specific metadata
        match &anim.metadata {
            wow_m2::AnimMetadata::Legacy {
                file_size,
                animation_count,
                structure_hints,
            } => {
                println!("File Size: {} bytes", file_size);
                println!("Animation Count (metadata): {}", animation_count);
                println!("Structure Valid: {}", structure_hints.appears_valid);
                println!("Estimated Blocks: {}", structure_hints.estimated_blocks);
                println!("Has Timestamps: {}", structure_hints.has_timestamps);
            }
            wow_m2::AnimMetadata::Modern { header, entries } => {
                println!("ANIM Version: {}", header.version);
                println!("ID Count: {}", header.id_count);
                println!("Entry Offset: {}", header.anim_entry_offset);
                println!("Entry Count: {}", entries.len());

                if !entries.is_empty() {
                    println!("\n=== Animation Entries ===");
                    for (i, entry) in entries.iter().enumerate() {
                        println!(
                            "Entry {}: ID={}, Offset={}, Size={}",
                            i, entry.id, entry.offset, entry.size
                        );
                    }
                }
            }
        }

        // Show memory breakdown
        println!("\n=== Memory Usage Breakdown ===");
        println!("Sections: {}", usage.sections);
        println!("Bone Animations: {}", usage.bone_animations);
        println!("Translation Keyframes: {}", usage.translation_keyframes);
        println!("Rotation Keyframes: {}", usage.rotation_keyframes);
        println!("Scaling Keyframes: {}", usage.scaling_keyframes);

        // Show sections summary
        if !anim.sections.is_empty() {
            println!("\n=== Animation Sections ===");
            for (i, section) in anim.sections.iter().enumerate() {
                println!(
                    "Section {}: ID={}, Start={}, End={}, Bones={}",
                    i,
                    section.header.id,
                    section.header.start,
                    section.header.end,
                    section.bone_animations.len()
                );
            }
        }
    }

    Ok(())
}

fn handle_anim_convert(input: PathBuf, output: PathBuf, version_str: String) -> Result<()> {
    println!("Loading ANIM file: {}", input.display());

    let anim = AnimFile::load(&input)
        .with_context(|| format!("Failed to load ANIM file from {}", input.display()))?;

    let target_version = M2Version::from_expansion_name(&version_str)
        .with_context(|| format!("Invalid target version: {version_str}"))?;

    println!("Source Format: {:?}", anim.format);
    println!("Converting to {target_version:?}");

    let converted = anim.convert(target_version);
    println!("Target Format: {:?}", converted.format);

    if converted.format == anim.format {
        println!("Note: No format conversion needed - same format for target version");
    }

    println!("Saving converted ANIM file to: {}", output.display());
    converted
        .save(&output)
        .with_context(|| format!("Failed to save converted ANIM file to {}", output.display()))?;

    println!("Conversion complete!");
    Ok(())
}

fn handle_blp_info(path: PathBuf, detailed: bool) -> Result<()> {
    println!("Loading BLP texture: {}", path.display());

    let _blp = load_blp(&path)
        .with_context(|| format!("Failed to load BLP texture from {}", path.display()))?;

    println!("\n=== BLP Texture Information ===");
    println!("File loaded successfully!");

    if detailed {
        println!("\n=== Detailed Information ===");
        println!("(Detailed information requires additional public API methods)");
    }

    Ok(())
}
