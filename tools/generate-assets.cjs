#!/usr/bin/env node
/**
 * Pixel Art Asset Generator using Google Gemini/Imagen API
 * Generates game assets with specific prompts for Code Warrior
 *
 * Usage: node tools/generate-assets.cjs [asset-type] [variant]
 * Example: node tools/generate-assets.cjs player down
 *          node tools/generate-assets.cjs tile grass
 */

const https = require('https');
const fs = require('fs');
const path = require('path');
const sharp = require('sharp');

// Configuration
const API_KEY = process.env.GEMINI_API_KEY || 'AIzaSyDGJFit3LUO-9mNkrqF1xTZVC71-yjGQlU';
const IMAGE_MODEL = 'imagen-4.0-generate-preview-06-06'; // Imagen 4 model
const OUTPUT_DIR = path.join(__dirname, '..', 'src-ui', 'static');

// Asset definitions with specific prompts
const ASSET_PROMPTS = {
    // Player sprites
    player: {
        down: {
            prompt: `32x32 pixel art sprite, knight hero facing forward, 16-bit SNES style, silver armor with gold trim, small sword at side, standing idle pose, transparent background, clean pixel edges, no anti-aliasing, limited 16 color palette`,
            output: 'sprites/player_down.png',
            size: 32
        },
        up: {
            prompt: `32x32 pixel art sprite, knight hero back view facing away, 16-bit SNES style, silver armor with red cape, gold trim, standing idle pose, transparent background, clean pixel edges, no anti-aliasing`,
            output: 'sprites/player_up.png',
            size: 32
        },
        left: {
            prompt: `32x32 pixel art sprite, knight hero side profile facing left, 16-bit SNES style, silver armor, sword at hip, gold trim, standing idle pose, transparent background, clean pixel edges`,
            output: 'sprites/player_left.png',
            size: 32
        },
        right: {
            prompt: `32x32 pixel art sprite, knight hero side profile facing right, 16-bit SNES style, silver armor, sword at hip, gold trim, standing idle pose, transparent background, clean pixel edges`,
            output: 'sprites/player_right.png',
            size: 32
        }
    },

    // NPC sprites
    npc: {
        mentor: {
            prompt: `32x32 pixel art sprite, wise old wizard mentor, 16-bit SNES RPG style, long white beard, purple robes, pointed hat, wooden staff with glowing blue crystal, standing pose facing forward, transparent background`,
            output: 'sprites/npc_mentor.png',
            size: 32
        }
    },

    // Terrain tiles
    tile: {
        grass: {
            prompt: `32x32 pixel art grass tile, top-down RPG view, Zelda Link to the Past style, various green shades, grass blade details, seamlessly tileable, 16-bit SNES era aesthetic`,
            output: 'tiles/terrain/grass.png',
            size: 32
        },
        stone: {
            prompt: `32x32 pixel art stone floor tile, top-down dungeon RPG, gray tones, brick pattern with mortar lines, weathered texture, seamlessly tileable, 16-bit SNES style`,
            output: 'tiles/terrain/stone.png',
            size: 32
        },
        water: {
            prompt: `32x32 pixel art water tile, top-down RPG, blue tones with white sparkle highlights, gentle wave pattern, seamlessly tileable, 16-bit SNES style`,
            output: 'tiles/terrain/water.png',
            size: 32
        },
        floor: {
            prompt: `32x32 pixel art basic floor tile, top-down dungeon RPG, light gray stone with subtle texture, clean and simple, seamlessly tileable, 16-bit SNES style`,
            output: 'tiles/floor.png',
            size: 32
        },
        void: {
            prompt: `32x32 pixel art void tile, top-down RPG, solid black or very dark navy blue representing empty space or abyss, minimal or no detail, 16-bit SNES style`,
            output: 'tiles/void.png',
            size: 32
        }
    },

    // Wall tiles
    wall: {
        stone: {
            prompt: `32x32 pixel art stone wall tile, top-down dungeon view, dark gray brick pattern, weathered texture with depth shading, 16-bit SNES dungeon aesthetic, tileable`,
            output: 'tiles/walls/stone_wall.png',
            size: 32
        }
    },

    // Interactive objects
    interactive: {
        terminal: {
            prompt: `32x32 pixel art magical ancient computer terminal, fantasy RPG style, stone pedestal with floating crystal screen, glowing cyan magical runes, mystical tech fusion, 16-bit SNES style`,
            output: 'tiles/interactive/terminal.png',
            size: 32
        },
        door_locked: {
            prompt: `32x32 pixel art locked wooden door, top-down RPG, heavy wood with iron bands, red glowing lock symbol, brown tones with metal gray, 16-bit SNES style`,
            output: 'tiles/interactive/door_locked.png',
            size: 32
        },
        door_open: {
            prompt: `32x32 pixel art open doorway, top-down RPG, stone frame with dark passage beyond, green glow indicating accessible, 16-bit SNES style`,
            output: 'tiles/interactive/door_open.png',
            size: 32
        }
    },

    // UI elements
    ui: {
        heart_full: {
            prompt: `16x16 pixel art heart icon, game HUD health indicator, Zelda style, bright red with pink highlight, clean symmetrical shape, transparent background, 8-bit style`,
            output: 'ui/heart_full.png',
            size: 16
        },
        heart_empty: {
            prompt: `16x16 pixel art empty heart outline, game HUD, dark gray outline with empty center, depleted health indicator, transparent background, 8-bit style`,
            output: 'ui/heart_empty.png',
            size: 16
        },
        gem: {
            prompt: `16x16 pixel art golden gem diamond, game HUD XP collectible, gold and amber tones with white sparkle highlight, faceted design, transparent background, 8-bit style`,
            output: 'ui/gem.png',
            size: 16
        }
    }
};

/**
 * Make API request to Imagen model
 */
async function callGeminiImageAPI(prompt) {
    // Imagen models use the predict endpoint with instances format
    const requestBody = {
        instances: [{ prompt: prompt }],
        parameters: {
            sampleCount: 1,
            aspectRatio: "1:1",
            personGeneration: "allow_adult",
            safetySetting: "block_low_and_above"
        }
    };

    const options = {
        hostname: 'generativelanguage.googleapis.com',
        path: `/v1beta/models/${IMAGE_MODEL}:predict?key=${API_KEY}`,
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        }
    };

    return new Promise((resolve, reject) => {
        const req = https.request(options, (res) => {
            let data = '';
            res.on('data', chunk => data += chunk);
            res.on('end', () => {
                try {
                    const response = JSON.parse(data);
                    if (response.error) {
                        reject(new Error(`API Error: ${response.error.message || JSON.stringify(response.error)}`));
                        return;
                    }
                    resolve(response);
                } catch (e) {
                    reject(new Error(`Failed to parse response: ${e.message}\nRaw: ${data.substring(0, 500)}`));
                }
            });
        });

        req.on('error', reject);
        req.write(JSON.stringify(requestBody));
        req.end();
    });
}

/**
 * Extract image data from Gemini response
 */
function extractImageFromResponse(response) {
    // Check for candidates with inline image data
    if (response.candidates && response.candidates[0]) {
        const candidate = response.candidates[0];
        if (candidate.content && candidate.content.parts) {
            for (const part of candidate.content.parts) {
                if (part.inlineData) {
                    return Buffer.from(part.inlineData.data, 'base64');
                }
            }
        }
    }

    // Check for predictions format (Imagen style)
    if (response.predictions && response.predictions[0]) {
        const prediction = response.predictions[0];
        if (prediction.bytesBase64Encoded) {
            return Buffer.from(prediction.bytesBase64Encoded, 'base64');
        }
    }

    throw new Error('No image data found in response: ' + JSON.stringify(response).substring(0, 800));
}

/**
 * Validate generated image
 */
function validateImage(imageBuffer, expectedSize) {
    const pngSignature = Buffer.from([0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]);
    if (!imageBuffer.slice(0, 8).equals(pngSignature)) {
        // Check if it's JPEG
        if (imageBuffer[0] === 0xFF && imageBuffer[1] === 0xD8) {
            return { valid: true, format: 'jpeg', note: 'JPEG format, may need conversion' };
        }
        return { valid: false, error: 'Not a valid PNG/JPEG file' };
    }

    const width = imageBuffer.readUInt32BE(16);
    const height = imageBuffer.readUInt32BE(20);

    // Imagen doesn't generate exact sizes, so we just note the actual size
    return {
        valid: true,
        width,
        height,
        note: width !== expectedSize ? `Generated ${width}x${height}, expected ${expectedSize}x${expectedSize} - will need resize` : null
    };
}

/**
 * Resize image to target size using sharp
 */
async function resizeImage(imageBuffer, targetSize) {
    return sharp(imageBuffer)
        .resize(targetSize, targetSize, {
            kernel: sharp.kernel.nearest, // Preserve pixel art look
            fit: 'fill'
        })
        .png()
        .toBuffer();
}

/**
 * Save image to disk (with optional resize)
 */
async function saveImage(imageBuffer, outputPath, targetSize = null) {
    const fullPath = path.join(OUTPUT_DIR, outputPath);
    const dir = path.dirname(fullPath);

    if (!fs.existsSync(dir)) {
        fs.mkdirSync(dir, { recursive: true });
    }

    let finalBuffer = imageBuffer;
    if (targetSize) {
        console.log(`Resizing to ${targetSize}x${targetSize}...`);
        finalBuffer = await resizeImage(imageBuffer, targetSize);
    }

    fs.writeFileSync(fullPath, finalBuffer);
    return fullPath;
}

/**
 * Generate a single asset
 */
async function generateAsset(category, variant) {
    const categoryAssets = ASSET_PROMPTS[category];
    if (!categoryAssets) {
        throw new Error(`Unknown category: ${category}. Available: ${Object.keys(ASSET_PROMPTS).join(', ')}`);
    }

    const assetConfig = categoryAssets[variant];
    if (!assetConfig) {
        throw new Error(`Unknown variant: ${variant}. Available for ${category}: ${Object.keys(categoryAssets).join(', ')}`);
    }

    console.log(`\nGenerating ${category}/${variant}...`);
    console.log(`Prompt: ${assetConfig.prompt.substring(0, 80)}...`);

    try {
        // Call Gemini Image Generation API
        console.log('Calling Gemini Image API...');
        const response = await callGeminiImageAPI(assetConfig.prompt);
        const imageBuffer = extractImageFromResponse(response);

        const validation = validateImage(imageBuffer, assetConfig.size);
        if (!validation.valid) {
            console.warn(`Warning: ${validation.error}`);
        } else {
            console.log(`Generated: ${validation.width}x${validation.height} pixels`);
        }

        // Always resize to target size for consistent pixel art
        const savedPath = await saveImage(imageBuffer, assetConfig.output, assetConfig.size);
        console.log(`Saved: ${savedPath} (${assetConfig.size}x${assetConfig.size})`);

        return { success: true, path: savedPath, validation };
    } catch (error) {
        console.error(`Failed: ${error.message}`);
        return { success: false, error: error.message };
    }
}

/**
 * Generate all assets of a category
 */
async function generateCategory(category) {
    const categoryAssets = ASSET_PROMPTS[category];
    if (!categoryAssets) {
        throw new Error(`Unknown category: ${category}`);
    }

    const results = [];
    for (const variant of Object.keys(categoryAssets)) {
        const result = await generateAsset(category, variant);
        results.push({ variant, ...result });
        // Rate limiting
        await new Promise(resolve => setTimeout(resolve, 2000));
    }

    return results;
}

/**
 * Generate all assets
 */
async function generateAll() {
    console.log('Generating all pixel art assets...\n');

    const allResults = {};
    for (const category of Object.keys(ASSET_PROMPTS)) {
        console.log(`\n=== Category: ${category.toUpperCase()} ===`);
        allResults[category] = await generateCategory(category);
    }

    // Summary
    console.log('\n\n=== GENERATION SUMMARY ===');
    let totalSuccess = 0;
    let totalFailed = 0;

    for (const [category, results] of Object.entries(allResults)) {
        const succeeded = results.filter(r => r.success).length;
        const failed = results.filter(r => !r.success).length;
        totalSuccess += succeeded;
        totalFailed += failed;
        console.log(`${category}: ${succeeded} succeeded, ${failed} failed`);
    }

    console.log(`\nTotal: ${totalSuccess} succeeded, ${totalFailed} failed`);
}

/**
 * List available assets
 */
function listAssets() {
    console.log('Available assets to generate:\n');
    for (const [category, variants] of Object.entries(ASSET_PROMPTS)) {
        console.log(`${category}:`);
        for (const [variant, config] of Object.entries(variants)) {
            console.log(`  - ${variant} (${config.size}x${config.size}) -> ${config.output}`);
        }
    }
}

/**
 * Show prompt for an asset
 */
function showPrompt(category, variant) {
    const categoryAssets = ASSET_PROMPTS[category];
    if (!categoryAssets) {
        console.error(`Unknown category: ${category}`);
        return;
    }
    const assetConfig = categoryAssets[variant];
    if (!assetConfig) {
        console.error(`Unknown variant: ${variant}`);
        return;
    }
    console.log(`\nPrompt for ${category}/${variant}:`);
    console.log('---');
    console.log(assetConfig.prompt);
    console.log('---');
    console.log(`Output: ${assetConfig.output}`);
    console.log(`Size: ${assetConfig.size}x${assetConfig.size}`);
}

// CLI interface
async function main() {
    const args = process.argv.slice(2);

    if (args.length === 0 || args[0] === '--help' || args[0] === '-h') {
        console.log(`
Pixel Art Asset Generator for Code Warrior
==========================================

Usage:
  node tools/generate-assets.cjs [command] [args]

Commands:
  list                      List all available assets
  prompt <category> <var>   Show the prompt for an asset
  all                       Generate all assets
  <category> <variant>      Generate specific asset
  <category>                Generate all variants of a category

Examples:
  node tools/generate-assets.cjs list
  node tools/generate-assets.cjs prompt player down
  node tools/generate-assets.cjs player down
  node tools/generate-assets.cjs tile
  node tools/generate-assets.cjs all

Categories: ${Object.keys(ASSET_PROMPTS).join(', ')}
        `);
        return;
    }

    if (args[0] === 'list') {
        listAssets();
        return;
    }

    if (args[0] === 'prompt') {
        showPrompt(args[1], args[2]);
        return;
    }

    if (args[0] === 'all') {
        await generateAll();
        return;
    }

    const category = args[0];
    const variant = args[1];

    if (variant) {
        await generateAsset(category, variant);
    } else {
        await generateCategory(category);
    }
}

main().catch(console.error);
