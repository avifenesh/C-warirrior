/** Particle system for visual effects. */

export interface Particle {
    x: number;
    y: number;
    vx: number;
    vy: number;
    lifetime: number;
    maxLifetime: number;
    color: string;
    size: number;
    alpha: number;
    gravity: number;
}

export type ParticlePreset = 'sparkle' | 'burst' | 'confetti' | 'smoke';

export class ParticleSystem {
    private particles: Particle[] = [];
    private maxParticles: number;

    constructor(maxParticles: number = 500) {
        this.maxParticles = maxParticles;
    }

    /** Emit particles at a position with optional config. */
    emit(
        x: number,
        y: number,
        count: number,
        preset: ParticlePreset = 'sparkle',
        config?: Partial<Particle>
    ): void {
        const presetConfig = this.getPresetConfig(preset);

        for (let i = 0; i < count && this.particles.length < this.maxParticles; i++) {
            const angle = Math.random() * Math.PI * 2;
            const speed = presetConfig.speed + Math.random() * presetConfig.speedVariation;

            const particle: Particle = {
                x,
                y,
                vx: Math.cos(angle) * speed,
                vy: Math.sin(angle) * speed,
                lifetime: 0,
                maxLifetime: presetConfig.lifetime + Math.random() * presetConfig.lifetimeVariation,
                color: presetConfig.colors[Math.floor(Math.random() * presetConfig.colors.length)],
                size: presetConfig.size + Math.random() * presetConfig.sizeVariation,
                alpha: 1,
                gravity: presetConfig.gravity,
                ...config,
            };

            this.particles.push(particle);
        }
    }

    /** Update all particles. Returns number of active particles. */
    update(deltaTime: number): number {
        const dt = deltaTime / 1000; // Convert to seconds

        this.particles = this.particles.filter((particle) => {
            // Update position
            particle.x += particle.vx * dt;
            particle.y += particle.vy * dt;
            particle.vy += particle.gravity * dt;

            // Update lifetime
            particle.lifetime += deltaTime;

            // Fade out near end of life
            const lifePercent = particle.lifetime / particle.maxLifetime;
            particle.alpha = 1 - lifePercent;

            // Remove dead particles
            return particle.lifetime < particle.maxLifetime;
        });

        return this.particles.length;
    }

    /** Render all particles to a canvas context. */
    render(ctx: CanvasRenderingContext2D, camera: { x: number; y: number }, tileSize: number): void {
        this.particles.forEach((particle) => {
            const screenX = particle.x - camera.x * tileSize;
            const screenY = particle.y - camera.y * tileSize;

            ctx.save();
            ctx.globalAlpha = particle.alpha;
            ctx.fillStyle = particle.color;

            // Draw particle as circle
            ctx.beginPath();
            ctx.arc(screenX, screenY, particle.size, 0, Math.PI * 2);
            ctx.fill();

            ctx.restore();
        });
    }

    /** Clear all particles. */
    clear(): void {
        this.particles = [];
    }

    /** Get number of active particles. */
    getCount(): number {
        return this.particles.length;
    }

    private getPresetConfig(preset: ParticlePreset) {
        switch (preset) {
            case 'sparkle':
                return {
                    speed: 30,
                    speedVariation: 20,
                    lifetime: 800,
                    lifetimeVariation: 400,
                    size: 2,
                    sizeVariation: 1,
                    gravity: -20,
                    colors: ['#fbbf24', '#fef08a', '#facc15', '#fb923c'],
                };
            case 'burst':
                return {
                    speed: 100,
                    speedVariation: 50,
                    lifetime: 600,
                    lifetimeVariation: 300,
                    size: 3,
                    sizeVariation: 2,
                    gravity: 100,
                    colors: ['#22d3ee', '#06b6d4', '#0ea5e9', '#3b82f6'],
                };
            case 'confetti':
                return {
                    speed: 80,
                    speedVariation: 40,
                    lifetime: 2000,
                    lifetimeVariation: 1000,
                    size: 4,
                    sizeVariation: 2,
                    gravity: 200,
                    colors: [
                        '#ef4444',
                        '#f59e0b',
                        '#eab308',
                        '#22c55e',
                        '#06b6d4',
                        '#3b82f6',
                        '#8b5cf6',
                        '#ec4899',
                    ],
                };
            case 'smoke':
                return {
                    speed: 20,
                    speedVariation: 10,
                    lifetime: 1500,
                    lifetimeVariation: 500,
                    size: 8,
                    sizeVariation: 4,
                    gravity: -30,
                    colors: ['#64748b', '#475569', '#334155', '#1e293b'],
                };
            default:
                return {
                    speed: 50,
                    speedVariation: 25,
                    lifetime: 1000,
                    lifetimeVariation: 500,
                    size: 3,
                    sizeVariation: 1,
                    gravity: 0,
                    colors: ['#ffffff'],
                };
        }
    }
}

/** Emit XP sparkles at a position. */
export function emitXpSparkles(system: ParticleSystem, x: number, y: number, xpAmount: number): void {
    const particleCount = Math.min(Math.floor(xpAmount / 10), 30);
    system.emit(x, y, particleCount, 'sparkle');
}

/** Emit code success burst. */
export function emitCodeSuccessBurst(system: ParticleSystem, x: number, y: number): void {
    system.emit(x, y, 50, 'burst');
}

/** Emit level complete confetti. */
export function emitLevelCompleteConfetti(
    system: ParticleSystem,
    centerX: number,
    centerY: number
): void {
    // Emit from multiple points for better spread
    for (let i = 0; i < 5; i++) {
        const offsetX = (Math.random() - 0.5) * 200;
        const offsetY = -100 + Math.random() * 50;
        system.emit(centerX + offsetX, centerY + offsetY, 40, 'confetti');
    }
}
