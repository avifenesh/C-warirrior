// Asset type definitions for Code Warrior

export interface SpriteFrame {
    x: number;
    y: number;
    width: number;
    height: number;
}

export interface SpriteSheet {
    image: string;
    frameWidth: number;
    frameHeight: number;
    frames: Record<string, SpriteFrame>;
    animations?: Record<string, {
        frames: string[];
        frameRate: number;
        loop: boolean;
    }>;
}

export interface TileDefinition {
    id: number;
    name: string;
    walkable: boolean;
    interactable: boolean;
    spriteX: number;
    spriteY: number;
}

export interface TileMap {
    width: number;
    height: number;
    tileWidth: number;
    tileHeight: number;
    tileset: string;
    tiles: TileDefinition[];
    layers: {
        name: string;
        data: number[];
    }[];
}

export interface AssetManifest {
    sprites: Record<string, string>;
    tiles: Record<string, string>;
    ui: Record<string, string>;
    fonts: Record<string, string>;
}

export const TILE_SIZE = 32;

export const DEFAULT_MANIFEST: AssetManifest = {
    sprites: {
        player: '/sprites/player.png'
    },
    tiles: {
        floor: '/tiles/floor.png',
        wall: '/tiles/wall.png',
        terminal: '/tiles/terminal.png'
    },
    ui: {},
    fonts: {}
};
