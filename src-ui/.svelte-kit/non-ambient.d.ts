
// this file is generated — do not edit it


declare module "svelte/elements" {
	export interface HTMLAttributes<T> {
		'data-sveltekit-keepfocus'?: true | '' | 'off' | undefined | null;
		'data-sveltekit-noscroll'?: true | '' | 'off' | undefined | null;
		'data-sveltekit-preload-code'?:
			| true
			| ''
			| 'eager'
			| 'viewport'
			| 'hover'
			| 'tap'
			| 'off'
			| undefined
			| null;
		'data-sveltekit-preload-data'?: true | '' | 'hover' | 'tap' | 'off' | undefined | null;
		'data-sveltekit-reload'?: true | '' | 'off' | undefined | null;
		'data-sveltekit-replacestate'?: true | '' | 'off' | undefined | null;
	}
}

export {};


declare module "$app/types" {
	export interface AppTypes {
		RouteId(): "/";
		RouteParams(): {
			
		};
		LayoutParams(): {
			"/": Record<string, never>
		};
		Pathname(): "/";
		ResolvedPathname(): `${"" | `/${string}`}${ReturnType<AppTypes['Pathname']>}`;
		Asset(): "/sprites/npc_mentor.png" | "/sprites/player.png" | "/sprites/player_down.png" | "/sprites/player_left.png" | "/sprites/player_right.png" | "/sprites/player_up.png" | "/tiles/door_locked.png" | "/tiles/door_open.png" | "/tiles/floor.png" | "/tiles/floor_stone.png" | "/tiles/floor_tech.png" | "/tiles/terminal.png" | "/tiles/void.png" | "/tiles/wall.png" | "/tiles/wall_top.png" | "/ui/health_bar_bg.png" | "/ui/health_bar_fill.png" | "/ui/terminal_frame.png" | "/ui/xp_bar_bg.png" | "/ui/xp_bar_fill.png" | string & {};
	}
}