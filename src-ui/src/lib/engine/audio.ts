export interface AudioManager {
    playSound(name: string): void;
    playMusic(name: string): void;
    stopMusic(): void;
    setVolume(volume: number): void;
    mute(): void;
    unmute(): void;
}

export const SOUNDS = {
    move: 'move',
    interact: 'interact',
    codeSuccess: 'code_success',
    codeFail: 'code_fail',
    doorOpen: 'door_open',
    levelComplete: 'level_complete',
} as const;

/** Create a minimal audio manager; starts muted and tolerates missing assets. */
export function createAudioManager(): AudioManager {
    const soundCache = new Map<string, HTMLAudioElement>();
    let music: HTMLAudioElement | null = null;
    let muted = true;
    let volume = 0.6;

    const safePlay = (audio?: HTMLAudioElement | null) => {
        if (!audio) return;
        audio.currentTime = 0;
        audio.volume = muted ? 0 : volume;
        audio.play().catch(() => undefined);
    };

    return {
        playSound(name: string) {
            const audio = soundCache.get(name);
            safePlay(audio);
        },
        playMusic(name: string) {
            const track = soundCache.get(name);
            if (!track) return;
            if (music && music !== track) {
                music.pause();
            }
            music = track;
            music.loop = true;
            music.volume = muted ? 0 : volume;
            music.play().catch(() => undefined);
        },
        stopMusic() {
            music?.pause();
            music = null;
        },
        setVolume(v: number) {
            volume = Math.max(0, Math.min(1, v));
            if (music) music.volume = muted ? 0 : volume;
        },
        mute() {
            muted = true;
            if (music) music.volume = 0;
        },
        unmute() {
            muted = false;
            if (music) music.volume = volume;
        },
    };
}
